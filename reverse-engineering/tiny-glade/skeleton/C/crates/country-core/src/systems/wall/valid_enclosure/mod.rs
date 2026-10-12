// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/wall/valid_enclosure/mod.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e6c66638cf88a4e97a2aadda181d5540520e6f2e73316941af308024055b0f36
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 214, 452, 503, 510, 520, 535, 538, 557, 561, 567, 568, 570, 574, 575, 578, 582, 583, 584, 585, 593, 594, 609, 1636, 1643, 1682, 1720, 1795, 1802, 2741, 3273, 3778, 3779, 4664
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
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":53616,"rva":13144416,"va":5381853536}
// 编译实例 RVAs: 0xc89160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34532745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381853536,"raw_hex":"e889ed0e02","size":5,"soft_fail":false,"target_va":5416386464,"va":5381853714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":53816,"parent_record_offset":53616,"ancestor_record_offset":53616,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":609,"record_offset":53836,"parent_record_offset":53816,"ancestor_record_offset":53616,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":609,"record_offset":53856,"parent_record_offset":53836,"ancestor_record_offset":53616,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":609,"record_offset":53880,"parent_record_offset":53856,"ancestor_record_offset":53616,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 11, 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":6208,"rva":13130768,"va":5381839888}
// 编译实例 RVAs: 0xc85c10, 0xc864f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t-6161526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839888,"raw_hex":"e88afba1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381840225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12501457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839888,"raw_hex":"e82f3e41ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5381840300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4e1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15498187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381842160,"raw_hex":"e8cb7bec00","size":5,"soft_fail":false,"target_va":5397340640,"va":5381842448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4e280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15498134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381842160,"raw_hex":"e8967bec00","size":5,"soft_fail":false,"target_va":5397340800,"va":5381842661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4e320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15500791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839888,"raw_hex":"e8f785ec00","size":5,"soft_fail":false,"target_va":5397340960,"va":5381840164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4e3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15500746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839888,"raw_hex":"e8ca85ec00","size":5,"soft_fail":false,"target_va":5397341120,"va":5381840369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b8e7e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15764363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839888,"raw_hex":"e88b8bf000","size":5,"soft_fail":false,"target_va":5397604320,"va":5381839952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bc8560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16001174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839888,"raw_hex":"e89628f400","size":5,"soft_fail":false,"target_va":5397841248,"va":5381840069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70660
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34514884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839888,"raw_hex":"e8c4a70e02","size":5,"soft_fail":false,"target_va":5416355424,"va":5381840535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34779321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381842160,"raw_hex":"e8b9b01202","size":5,"soft_fail":false,"target_va":5416622240,"va":5381842914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t14640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839888,"raw_hex":"e830390000","size":5,"soft_fail":false,"target_va":5381854896,"va":5381840251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1463132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381842160,"raw_hex":"e85c531600","size":5,"soft_fail":false,"target_va":5383305920,"va":5381842783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":8064,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":609,"record_offset":6400,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":38817,"source_line":275,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":6208,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":6208,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":6208,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":6944,"parent_record_offset":6912,"ancestor_record_offset":6208,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":609,"record_offset":6912,"parent_record_offset":6884,"ancestor_record_offset":6208,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":609,"record_offset":8100,"parent_record_offset":8064,"ancestor_record_offset":6208,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":6732,"parent_record_offset":6400,"ancestor_record_offset":6208,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":6696,"parent_record_offset":6400,"ancestor_record_offset":6208,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":6632,"parent_record_offset":6400,"ancestor_record_offset":6208,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":609,"record_offset":6668,"parent_record_offset":6400,"ancestor_record_offset":6208,"inlinee":38817,"source_line":275,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":609,"record_offset":8128,"parent_record_offset":8100,"ancestor_record_offset":6208,"inlinee":33117,"source_line":46,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$2::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":609,"record_offset":14940,"parent_record_offset":14908,"ancestor_record_offset":13012,"inlinee":33119,"source_line":290,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":609,"record_offset":6436,"parent_record_offset":6400,"ancestor_record_offset":6208,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$21::visit_bool
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":609,"record_offset":6592,"parent_record_offset":6492,"ancestor_record_offset":6208,"inlinee":10035,"source_line":320,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$21::visit_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":609,"record_offset":6564,"parent_record_offset":6492,"ancestor_record_offset":6208,"inlinee":10057,"source_line":418,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":609,"record_offset":6464,"parent_record_offset":6436,"ancestor_record_offset":6208,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$8::deserialize_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/number.rs
//     evidence={"module":609,"record_offset":6536,"parent_record_offset":6492,"ancestor_record_offset":6208,"inlinee":21146,"source_line":538,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":609,"record_offset":6492,"parent_record_offset":6464,"ancestor_record_offset":6208,"inlinee":21143,"source_line":286,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::private::de::missing_field::MissingFieldDeserializer<serde_json::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":8276,"rva":47691168,"va":5416400288}
// 编译实例 RVAs: 0x2d7b5a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d706e0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-44793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416400288,"raw_hex":"e80751ffff","size":5,"soft_fail":false,"target_va":5416355552,"va":5416400340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":8656,"parent_record_offset":8276,"ancestor_record_offset":8276,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":609,"record_offset":8520,"parent_record_offset":8276,"ancestor_record_offset":8276,"inlinee":38817,"source_line":275,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":609,"record_offset":8540,"parent_record_offset":8520,"ancestor_record_offset":8276,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":609,"record_offset":8588,"parent_record_offset":8564,"ancestor_record_offset":8276,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":609,"record_offset":8564,"parent_record_offset":8540,"ancestor_record_offset":8276,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":609,"record_offset":8612,"parent_record_offset":8588,"ancestor_record_offset":8276,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 11, 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x40F0
// 首条位置: {"module":1636,"inlinee":16624}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=4
// 源码行: 11, 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":214,"record_offset":19700,"rva":5911776,"va":5374620896}
// 编译实例 RVAs: 0x5a34e0, 0x5a3750, 0xc85750, 0xc85f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1401962e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4248819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374621520,"raw_hex":"e80d2bbfff","size":5,"soft_fail":false,"target_va":5370372832,"va":5374621646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140248100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3520325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374621520,"raw_hex":"e8bb48caff","size":5,"soft_fail":false,"target_va":5371101440,"va":5374621760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140464120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1307836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374620896,"raw_hex":"e8440becff","size":5,"soft_fail":false,"target_va":5373313312,"va":5374621143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14046b660
//     owner=unresolved_namespace; count=8
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8495422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381838672,"raw_hex":"e8c25e7eff","size":5,"soft_fail":false,"target_va":5373343328,"va":5381838745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417db940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11884861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381840768,"raw_hex":"e83d59b500","size":5,"soft_fail":false,"target_va":5393725760,"va":5381840894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b9770
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12795684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381838672,"raw_hex":"e8243fc300","size":5,"soft_fail":false,"target_va":5394634608,"va":5381838919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141af5370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15135483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381840768,"raw_hex":"e8fbf2e600","size":5,"soft_fail":false,"target_va":5396976496,"va":5381841008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=20
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34513959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381838672,"raw_hex":"e827a40e02","size":5,"soft_fail":false,"target_va":5416353040,"va":5381839076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34783002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381838672,"raw_hex":"e81abf1202","size":5,"soft_fail":false,"target_va":5416622208,"va":5381839201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t34721107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381838672,"raw_hex":"e853cd1102","size":5,"soft_fail":false,"target_va":5416560208,"va":5381839096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tjmp\t34721162","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381838672,"raw_hex":"e98acd1102","size":5,"soft_fail":false,"target_va":5416560208,"va":5381839041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$10::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":21920,"parent_record_offset":19700,"ancestor_record_offset":19700,"inlinee":7412,"source_line":752,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$10::serialize_field
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":21864,"parent_record_offset":19700,"ancestor_record_offset":19700,"inlinee":7411,"source_line":738,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_newtype_variant
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":23216,"parent_record_offset":22936,"ancestor_record_offset":22936,"inlinee":7399,"source_line":233,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct_variant
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":20232,"parent_record_offset":19700,"ancestor_record_offset":19700,"inlinee":7409,"source_line":383,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":19992,"parent_record_offset":19700,"ancestor_record_offset":19700,"inlinee":7408,"source_line":214,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":214,"record_offset":20632,"parent_record_offset":20600,"ancestor_record_offset":19700,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":214,"record_offset":20500,"parent_record_offset":20472,"ancestor_record_offset":19700,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":214,"record_offset":20560,"parent_record_offset":20528,"ancestor_record_offset":19700,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":214,"record_offset":20680,"parent_record_offset":20656,"ancestor_record_offset":19700,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":214,"record_offset":20656,"parent_record_offset":20632,"ancestor_record_offset":19700,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":214,"record_offset":20528,"parent_record_offset":20500,"ancestor_record_offset":19700,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":214,"record_offset":20432,"parent_record_offset":20404,"ancestor_record_offset":19700,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":214,"record_offset":20600,"parent_record_offset":20432,"ancestor_record_offset":19700,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":214,"record_offset":20376,"parent_record_offset":20348,"ancestor_record_offset":19700,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":214,"record_offset":20720,"parent_record_offset":20432,"ancestor_record_offset":19700,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":214,"record_offset":20472,"parent_record_offset":20432,"ancestor_record_offset":19700,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":214,"record_offset":20404,"parent_record_offset":20376,"ancestor_record_offset":19700,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":214,"record_offset":20192,"parent_record_offset":20160,"ancestor_record_offset":19700,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":214,"record_offset":20160,"parent_record_offset":20132,"ancestor_record_offset":19700,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":20292,"parent_record_offset":20232,"ancestor_record_offset":19700,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":20868,"parent_record_offset":20232,"ancestor_record_offset":19700,"inlinee":7401,"source_line":1910,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":21968,"parent_record_offset":21920,"ancestor_record_offset":19700,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":21348,"parent_record_offset":20232,"ancestor_record_offset":19700,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=git_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":20016,"parent_record_offset":19992,"ancestor_record_offset":19700,"inlinee":7400,"source_line":190,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":21888,"parent_record_offset":21864,"ancestor_record_offset":19700,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":214,"record_offset":20348,"parent_record_offset":20320,"ancestor_record_offset":19700,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":214,"record_offset":20320,"parent_record_offset":20292,"ancestor_record_offset":19700,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":4376,"rva":13130176,"va":5381839296}
// 编译实例 RVAs: 0xc859c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeStructVariant$u20$as$u20$serde_core..ser..SerializeStructVariant$GT$::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1386134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839296,"raw_hex":"e896261500","size":5,"soft_fail":false,"target_va":5383225888,"va":5381839749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde_core..ser..Serializer$GT$::serialize_struct_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1387468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839296,"raw_hex":"e8cc2b1500","size":5,"soft_fail":false,"target_va":5383227008,"va":5381839535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6160683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839296,"raw_hex":"e8d5fea1ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381839366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6160954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839296,"raw_hex":"e8c6fda1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381839653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13117164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839296,"raw_hex":"e814d937ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381839351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140973dc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3218918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839296,"raw_hex":"e81ae2ceff","size":5,"soft_fail":false,"target_va":5378620864,"va":5381839777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409979d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3072570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839296,"raw_hex":"e8c61dd1ff","size":5,"soft_fail":false,"target_va":5378767312,"va":5381839877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b7c1b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15689387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839296,"raw_hex":"e8ab66ef00","size":5,"soft_fail":false,"target_va":5397529008,"va":5381839616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34780983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381839296,"raw_hex":"e837b71202","size":5,"soft_fail":false,"target_va":5416620794,"va":5381839806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":5728,"parent_record_offset":4376,"ancestor_record_offset":4376,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":5640,"parent_record_offset":4376,"ancestor_record_offset":4376,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":5672,"parent_record_offset":4376,"ancestor_record_offset":4376,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$1::serialize_unit_variant
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":609,"record_offset":4576,"parent_record_offset":4376,"ancestor_record_offset":4376,"inlinee":7422,"source_line":188,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":4920,"parent_record_offset":4896,"ancestor_record_offset":4376,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":4944,"parent_record_offset":4920,"ancestor_record_offset":4376,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":4376,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":4376,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":4968,"parent_record_offset":4944,"ancestor_record_offset":4376,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":5980,"parent_record_offset":5956,"ancestor_record_offset":4376,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":4896,"parent_record_offset":4868,"ancestor_record_offset":4376,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":5908,"parent_record_offset":5852,"ancestor_record_offset":4376,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":609,"record_offset":6096,"parent_record_offset":6068,"ancestor_record_offset":4376,"inlinee":4643,"source_line":207,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":4808,"parent_record_offset":4780,"ancestor_record_offset":4376,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":5880,"parent_record_offset":5852,"ancestor_record_offset":4376,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":5852,"parent_record_offset":5828,"ancestor_record_offset":4376,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":4868,"parent_record_offset":4836,"ancestor_record_offset":4376,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":4836,"parent_record_offset":4808,"ancestor_record_offset":4376,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":4376,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":609,"record_offset":4688,"parent_record_offset":4660,"ancestor_record_offset":4376,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":609,"record_offset":4716,"parent_record_offset":4688,"ancestor_record_offset":4376,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":609,"record_offset":4744,"parent_record_offset":4716,"ancestor_record_offset":4376,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":609,"record_offset":4660,"parent_record_offset":4632,"ancestor_record_offset":4376,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":609,"record_offset":4632,"parent_record_offset":4604,"ancestor_record_offset":4376,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":609,"record_offset":4780,"parent_record_offset":4744,"ancestor_record_offset":4376,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":5024,"parent_record_offset":4744,"ancestor_record_offset":4376,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":4376,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":5756,"parent_record_offset":5728,"ancestor_record_offset":4376,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":6124,"parent_record_offset":6096,"ancestor_record_offset":4376,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_str
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":609,"record_offset":4604,"parent_record_offset":4576,"ancestor_record_offset":4376,"inlinee":7423,"source_line":168,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$3::clone
// 记录形式: inline_instance, inlinee_declaration; instances=48
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7776
// 首条位置: {"module":520,"inlinee":30582}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$4::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":575,"record_offset":101152,"rva":12126512,"va":5380835632}
// 编译实例 RVAs: 0xb90930, 0x2827330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct_field1_finish
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3985406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380835632,"raw_hex":"e8fecf3c00","size":5,"soft_fail":false,"target_va":5384821184,"va":5380835773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t3988391","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5380835632,"raw_hex":"e9a7db3c00","size":5,"soft_fail":false,"target_va":5384824080,"va":5380835684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 22, 32, 38, 263, 269, 275, 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97A1
// 首条位置: {"module":609,"inlinee":38817}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":6732,"parent_record_offset":6400,"ancestor_record_offset":6400,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":6696,"parent_record_offset":6400,"ancestor_record_offset":6400,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":6632,"parent_record_offset":6400,"ancestor_record_offset":6400,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":609,"record_offset":6668,"parent_record_offset":6400,"ancestor_record_offset":6400,"inlinee":38817,"source_line":275,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":609,"record_offset":6436,"parent_record_offset":6400,"ancestor_record_offset":6400,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":6400,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":6400,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":6400,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":6944,"parent_record_offset":6912,"ancestor_record_offset":6400,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":609,"record_offset":6912,"parent_record_offset":6884,"ancestor_record_offset":6400,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":6764,"parent_record_offset":6732,"ancestor_record_offset":6400,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$21::visit_bool
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":609,"record_offset":6592,"parent_record_offset":6492,"ancestor_record_offset":6400,"inlinee":10035,"source_line":320,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$21::visit_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":609,"record_offset":6564,"parent_record_offset":6492,"ancestor_record_offset":6400,"inlinee":10057,"source_line":418,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::__deserialize_content_v1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":609,"record_offset":11416,"parent_record_offset":11392,"ancestor_record_offset":11352,"inlinee":38818,"source_line":2433,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":609,"record_offset":8588,"parent_record_offset":8564,"ancestor_record_offset":8520,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":609,"record_offset":6464,"parent_record_offset":6436,"ancestor_record_offset":6400,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":609,"record_offset":8612,"parent_record_offset":8588,"ancestor_record_offset":8520,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$8::deserialize_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/number.rs
//     evidence={"module":609,"record_offset":6536,"parent_record_offset":6492,"ancestor_record_offset":6400,"inlinee":21146,"source_line":538,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":609,"record_offset":6492,"parent_record_offset":6464,"ancestor_record_offset":6400,"inlinee":21143,"source_line":286,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=3
// 源码行: 22, 32, 269
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":52668,"rva":13143968,"va":5381853088}
// 编译实例 RVAs: 0xc88fa0, 0xc89370, 0xc89480
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-6174767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381853088,"raw_hex":"e8d1c7a1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381853466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77fa0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34533025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381853088,"raw_hex":"e8a1ee0e02","size":5,"soft_fail":false,"target_va":5416386464,"va":5381853434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::private::string::from_utf8_lossy
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2765487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381853088,"raw_hex":"e8af322a00","size":5,"soft_fail":false,"target_va":5384618896,"va":5381853404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":52868,"parent_record_offset":52668,"ancestor_record_offset":52668,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":53036,"parent_record_offset":53012,"ancestor_record_offset":52668,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":53060,"parent_record_offset":53036,"ancestor_record_offset":52668,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":53084,"parent_record_offset":53060,"ancestor_record_offset":52668,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":53012,"parent_record_offset":52988,"ancestor_record_offset":52668,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":52988,"parent_record_offset":52964,"ancestor_record_offset":52668,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":52964,"parent_record_offset":52940,"ancestor_record_offset":52668,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":52892,"parent_record_offset":52868,"ancestor_record_offset":52668,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=22
// 源码行: 22, 32, 263, 269, 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97A0
// 首条位置: {"module":609,"inlinee":38816}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 22, 32, 263, 269, 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155F9
// 首条位置: {"module":1795,"inlinee":87545}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":1952,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":1924,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":1924,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":1924,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":2280,"parent_record_offset":2256,"ancestor_record_offset":1924,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":2256,"parent_record_offset":1952,"ancestor_record_offset":1924,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1924,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":1924,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":2060,"parent_record_offset":1952,"ancestor_record_offset":1924,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1795,"record_offset":16812,"parent_record_offset":16788,"ancestor_record_offset":16476,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":16836,"parent_record_offset":16812,"ancestor_record_offset":16476,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1795,"record_offset":16788,"parent_record_offset":16764,"ancestor_record_offset":16476,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1795,"record_offset":17156,"parent_record_offset":17120,"ancestor_record_offset":16476,"inlinee":38859,"source_line":275,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1795,"record_offset":16764,"parent_record_offset":16724,"ancestor_record_offset":16476,"inlinee":38860,"source_line":275,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1795,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":1924,"inlinee":38816,"source_line":275,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":17120,"parent_record_offset":16536,"ancestor_record_offset":16476,"inlinee":10116,"source_line":1602,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":16724,"parent_record_offset":16536,"ancestor_record_offset":16476,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":2008,"parent_record_offset":1952,"ancestor_record_offset":1924,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 22, 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97CE
// 首条位置: {"module":1795,"inlinee":38862}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 22, 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97CF
// 首条位置: {"module":1795,"inlinee":38863}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1795,"record_offset":74792,"parent_record_offset":74764,"ancestor_record_offset":74764,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1795,"record_offset":74820,"parent_record_offset":74792,"ancestor_record_offset":74764,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":74848,"parent_record_offset":74820,"ancestor_record_offset":74764,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":74876,"parent_record_offset":74848,"ancestor_record_offset":74764,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 22, 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x979E
// 首条位置: {"module":609,"inlinee":38814}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 22, 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1560E
// 首条位置: {"module":1795,"inlinee":87566}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":74600,"parent_record_offset":74560,"ancestor_record_offset":74560,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":75216,"parent_record_offset":75192,"ancestor_record_offset":74560,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":75264,"parent_record_offset":75240,"ancestor_record_offset":74560,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":75240,"parent_record_offset":75216,"ancestor_record_offset":74560,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":75144,"parent_record_offset":75120,"ancestor_record_offset":74560,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":75120,"parent_record_offset":74600,"ancestor_record_offset":74560,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":75192,"parent_record_offset":75168,"ancestor_record_offset":74560,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":75168,"parent_record_offset":75144,"ancestor_record_offset":74560,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1795,"record_offset":74820,"parent_record_offset":74792,"ancestor_record_offset":74560,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":74848,"parent_record_offset":74820,"ancestor_record_offset":74560,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":74876,"parent_record_offset":74848,"ancestor_record_offset":74560,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1795,"record_offset":74792,"parent_record_offset":74764,"ancestor_record_offset":74560,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1795,"record_offset":74948,"parent_record_offset":74924,"ancestor_record_offset":74560,"inlinee":38862,"source_line":32,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1795,"record_offset":74764,"parent_record_offset":74736,"ancestor_record_offset":74560,"inlinee":38863,"source_line":32,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1795,"record_offset":74656,"parent_record_offset":74600,"ancestor_record_offset":74560,"inlinee":38814,"source_line":32,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":74924,"parent_record_offset":74600,"ancestor_record_offset":74560,"inlinee":10116,"source_line":1602,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":74736,"parent_record_offset":74600,"ancestor_record_offset":74560,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":74680,"parent_record_offset":74600,"ancestor_record_offset":74560,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 22, 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1560C
// 首条位置: {"module":1795,"inlinee":87564}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1795,"record_offset":75340,"parent_record_offset":74240,"ancestor_record_offset":74240,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1795,"record_offset":75480,"parent_record_offset":74240,"ancestor_record_offset":74240,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":75512,"parent_record_offset":74240,"ancestor_record_offset":74240,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":75572,"parent_record_offset":74240,"ancestor_record_offset":74240,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":74296,"parent_record_offset":74240,"ancestor_record_offset":74240,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":75368,"parent_record_offset":74240,"ancestor_record_offset":74240,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":75216,"parent_record_offset":75192,"ancestor_record_offset":74240,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":75264,"parent_record_offset":75240,"ancestor_record_offset":74240,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":75240,"parent_record_offset":75216,"ancestor_record_offset":74240,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":75144,"parent_record_offset":75120,"ancestor_record_offset":74240,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":75120,"parent_record_offset":74600,"ancestor_record_offset":74240,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":75192,"parent_record_offset":75168,"ancestor_record_offset":74240,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":75168,"parent_record_offset":75144,"ancestor_record_offset":74240,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1795,"record_offset":74820,"parent_record_offset":74792,"ancestor_record_offset":74240,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1795,"record_offset":74480,"parent_record_offset":74452,"ancestor_record_offset":74240,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":74848,"parent_record_offset":74820,"ancestor_record_offset":74240,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":74876,"parent_record_offset":74848,"ancestor_record_offset":74240,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1795,"record_offset":74452,"parent_record_offset":74424,"ancestor_record_offset":74240,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1795,"record_offset":74792,"parent_record_offset":74764,"ancestor_record_offset":74240,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1795,"record_offset":74948,"parent_record_offset":74924,"ancestor_record_offset":74240,"inlinee":38862,"source_line":32,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1795,"record_offset":74764,"parent_record_offset":74736,"ancestor_record_offset":74240,"inlinee":38863,"source_line":32,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1795,"record_offset":74656,"parent_record_offset":74600,"ancestor_record_offset":74240,"inlinee":38814,"source_line":32,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1795,"record_offset":74560,"parent_record_offset":74520,"ancestor_record_offset":74240,"inlinee":87566,"source_line":32,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::MapRefDeserializer::next_pair
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":74424,"parent_record_offset":74376,"ancestor_record_offset":74240,"inlinee":11185,"source_line":2584,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":74600,"parent_record_offset":74560,"ancestor_record_offset":74240,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_key_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":74376,"parent_record_offset":74336,"ancestor_record_offset":74240,"inlinee":11183,"source_line":2642,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":75416,"parent_record_offset":75392,"ancestor_record_offset":74240,"inlinee":11186,"source_line":2655,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":74336,"parent_record_offset":74296,"ancestor_record_offset":74240,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":75392,"parent_record_offset":75368,"ancestor_record_offset":74240,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":74924,"parent_record_offset":74600,"ancestor_record_offset":74240,"inlinee":10116,"source_line":1602,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":74736,"parent_record_offset":74600,"ancestor_record_offset":74240,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":74680,"parent_record_offset":74600,"ancestor_record_offset":74240,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":74520,"parent_record_offset":74376,"ancestor_record_offset":74240,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 22, 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1560F
// 首条位置: {"module":1795,"inlinee":87567}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":75904,"parent_record_offset":75872,"ancestor_record_offset":75872,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1795,"record_offset":76016,"parent_record_offset":75992,"ancestor_record_offset":75872,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1795,"record_offset":76076,"parent_record_offset":75960,"ancestor_record_offset":75872,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1795,"record_offset":75992,"parent_record_offset":75960,"ancestor_record_offset":75872,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":75960,"parent_record_offset":75932,"ancestor_record_offset":75872,"inlinee":15023,"source_line":2527,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":75932,"parent_record_offset":75904,"ancestor_record_offset":75872,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":76048,"parent_record_offset":75960,"ancestor_record_offset":75872,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum<serde::private::de::content::EnumRefDeserializer<serde_json::error::Error> >
// 记录形式: compiled_procedure; instances=3
// 源码行: 22, 263, 269
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":51288,"rva":13143536,"va":5381852656}
// 编译实例 RVAs: 0xc88df0, 0xc88ea0, 0xc89230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t34707469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381852832,"raw_hex":"e80d981102","size":5,"soft_fail":false,"target_va":5416560512,"va":5381853038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4d760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15485102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381852832,"raw_hex":"e8ae48ec00","size":5,"soft_fail":false,"target_va":5397337952,"va":5381852845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4d920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15485726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381852656,"raw_hex":"e81e4bec00","size":5,"soft_fail":false,"target_va":5397338400,"va":5381852669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4da50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15484942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381853744,"raw_hex":"e80e48ec00","size":5,"soft_fail":false,"target_va":5397338704,"va":5381853757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4e550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15488583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381852832,"raw_hex":"e84756ec00","size":5,"soft_fail":false,"target_va":5397341520,"va":5381852932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4f5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15491859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381853744,"raw_hex":"e81363ec00","size":5,"soft_fail":false,"target_va":5397345712,"va":5381853848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b50120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15494709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381853744,"raw_hex":"e8356eec00","size":5,"soft_fail":false,"target_va":5397348640,"va":5381853926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b57670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15525929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381852656,"raw_hex":"e829e8ec00","size":5,"soft_fail":false,"target_va":5397378672,"va":5381852738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd90c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34931238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381852656,"raw_hex":"e826021502","size":5,"soft_fail":false,"target_va":5416784064,"va":5381852821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":52492,"parent_record_offset":51984,"ancestor_record_offset":51984,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":51580,"parent_record_offset":51288,"ancestor_record_offset":51288,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":51692,"parent_record_offset":51288,"ancestor_record_offset":51288,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$52::unit_variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":609,"record_offset":51608,"parent_record_offset":51288,"ancestor_record_offset":51288,"inlinee":6485,"source_line":2849,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":609,"record_offset":51552,"parent_record_offset":51288,"ancestor_record_offset":51288,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":609,"record_offset":52368,"parent_record_offset":51984,"ancestor_record_offset":51984,"inlinee":6481,"source_line":2180,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":609,"record_offset":52456,"parent_record_offset":52432,"ancestor_record_offset":51984,"inlinee":38817,"source_line":275,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_unit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":609,"record_offset":51656,"parent_record_offset":51632,"ancestor_record_offset":51288,"inlinee":6489,"source_line":2266,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$52::newtype_variant_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":609,"record_offset":52396,"parent_record_offset":52368,"ancestor_record_offset":51984,"inlinee":9610,"source_line":2860,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":609,"record_offset":52432,"parent_record_offset":52396,"ancestor_record_offset":51984,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$1::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":609,"record_offset":51632,"parent_record_offset":51608,"ancestor_record_offset":51288,"inlinee":6487,"source_line":32,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":609,"record_offset":54996,"parent_record_offset":54972,"ancestor_record_offset":54556,"inlinee":38829,"source_line":16,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":609,"record_offset":55148,"parent_record_offset":55124,"ancestor_record_offset":54556,"inlinee":38830,"source_line":114,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97CA
// 首条位置: {"module":1795,"inlinee":38858}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1795,"record_offset":15096,"parent_record_offset":15044,"ancestor_record_offset":15044,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1795,"record_offset":15124,"parent_record_offset":15044,"ancestor_record_offset":15044,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$52::unit_variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":15152,"parent_record_offset":15044,"ancestor_record_offset":15044,"inlinee":6485,"source_line":2849,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":15068,"parent_record_offset":15044,"ancestor_record_offset":15044,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_unit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":15208,"parent_record_offset":15180,"ancestor_record_offset":15044,"inlinee":6489,"source_line":2266,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":15180,"parent_record_offset":15152,"ancestor_record_offset":15044,"inlinee":6487,"source_line":32,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$0::from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x815D
// 首条位置: {"module":609,"inlinee":33117}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::private::de::content::ContentRefDeserializer<serde_json::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":11116,"rva":13132176,"va":5381841296}
// 编译实例 RVAs: 0xc86190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-6162808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381841296,"raw_hex":"e888f6a1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381841507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12502754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381841296,"raw_hex":"e81e3941ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5381841597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4e1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15499189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381841296,"raw_hex":"e8b57fec00","size":5,"soft_fail":false,"target_va":5397340640,"va":5381841446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4e280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15499136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381841296,"raw_hex":"e8807fec00","size":5,"soft_fail":false,"target_va":5397340800,"va":5381841659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70660
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34513551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381841296,"raw_hex":"e88fa20e02","size":5,"soft_fail":false,"target_va":5416355424,"va":5381841868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34780325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381841296,"raw_hex":"e8a5b41202","size":5,"soft_fail":false,"target_va":5416622240,"va":5381841910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t13356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381841296,"raw_hex":"e82c340000","size":5,"soft_fail":false,"target_va":5381854896,"va":5381841535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde::private::de::content::content_clone
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2775933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381841296,"raw_hex":"e87d5b2a00","size":5,"soft_fail":false,"target_va":5384617280,"va":5381841342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1464130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381841296,"raw_hex":"e842571600","size":5,"soft_fail":false,"target_va":5383305920,"va":5381841785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":12872,"parent_record_offset":11116,"ancestor_record_offset":11116,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":609,"record_offset":11352,"parent_record_offset":11116,"ancestor_record_offset":11116,"inlinee":38817,"source_line":275,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":11776,"parent_record_offset":11752,"ancestor_record_offset":11116,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":11800,"parent_record_offset":11776,"ancestor_record_offset":11116,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":11824,"parent_record_offset":11800,"ancestor_record_offset":11116,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":11752,"parent_record_offset":11720,"ancestor_record_offset":11116,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":609,"record_offset":11720,"parent_record_offset":11692,"ancestor_record_offset":11116,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":609,"record_offset":12912,"parent_record_offset":12872,"ancestor_record_offset":11116,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":11540,"parent_record_offset":11352,"ancestor_record_offset":11116,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":11504,"parent_record_offset":11352,"ancestor_record_offset":11116,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":11448,"parent_record_offset":11352,"ancestor_record_offset":11116,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":609,"record_offset":11480,"parent_record_offset":11352,"ancestor_record_offset":11116,"inlinee":38817,"source_line":275,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$2::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":609,"record_offset":12944,"parent_record_offset":12912,"ancestor_record_offset":11116,"inlinee":33119,"source_line":290,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":609,"record_offset":11392,"parent_record_offset":11352,"ancestor_record_offset":11116,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::__deserialize_content_v1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":609,"record_offset":11416,"parent_record_offset":11392,"ancestor_record_offset":11116,"inlinee":38818,"source_line":2433,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$10::eq
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89A0
// 首条位置: {"module":561,"inlinee":35232}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":32148,"parent_record_offset":32116,"ancestor_record_offset":32116,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$103::eq
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":32208,"parent_record_offset":32172,"ancestor_record_offset":32116,"inlinee":14672,"source_line":20,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":3779,"record_offset":4740,"parent_record_offset":4680,"ancestor_record_offset":4500,"inlinee":10600,"source_line":7,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::impl$7::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":561,"record_offset":32296,"parent_record_offset":32272,"ancestor_record_offset":32116,"inlinee":35238,"source_line":16,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$12::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":561,"record_offset":32172,"parent_record_offset":32148,"ancestor_record_offset":32116,"inlinee":35234,"source_line":114,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$6::clone
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A11
// 首条位置: {"module":452,"inlinee":27153}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$7::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8EF3
// 首条位置: {"module":575,"record_offset":101428,"rva":12126720,"va":5380835840}
// 编译实例 RVAs: 0xb90a00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_tuple_field1_finish
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3981765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380835840,"raw_hex":"e8c5c13c00","size":5,"soft_fail":false,"target_va":5384817664,"va":5380835894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::center
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=16
// 源码行: 82, 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7269
// 首条位置: {"module":510,"inlinee":29289}
// 编译实例 RVAs: 0xa623d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8141
// 首条位置: {"module":538,"record_offset":99232,"rva":10888272,"va":5379597392}
// 编译实例 RVAs: 0xa62450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3709067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379597392,"raw_hex":"e88b983800","size":5,"soft_fail":false,"target_va":5383306512,"va":5379597440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3724212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379597392,"raw_hex":"e8b4d33800","size":5,"soft_fail":false,"target_va":5383321632,"va":5379597415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3
// 记录形式: compiled_procedure; instances=1
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":99380,"rva":10888336,"va":5379597456}
// 编译实例 RVAs: 0xa62490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3709285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379597456,"raw_hex":"e865993800","size":5,"soft_fail":false,"target_va":5383306800,"va":5379597510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3724878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379597456,"raw_hex":"e84ed63800","size":5,"soft_fail":false,"target_va":5383322368,"va":5379597485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::contains_point
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=20
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FED
// 首条位置: {"module":503,"inlinee":28653}
// 编译实例 RVAs: 0xa620a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t3716747","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379596448,"raw_hex":"e98bb63800","size":5,"soft_fail":false,"target_va":5383313216,"va":5379596464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape
// 记录形式: compiled_procedure; instances=1
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":98312,"rva":10887360,"va":5379596480}
// 编译实例 RVAs: 0xa620c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::contains_point
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3707652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379596480,"raw_hex":"e804933800","size":5,"soft_fail":false,"target_va":5383304336,"va":5379596679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3707130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379596480,"raw_hex":"e8fa903800","size":5,"soft_fail":false,"target_va":5383303936,"va":5379596801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3716675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379596480,"raw_hex":"e843b63800","size":5,"soft_fail":false,"target_va":5383313216,"va":5379596536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3726689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379596480,"raw_hex":"e861dd3800","size":5,"soft_fail":false,"target_va":5383323216,"va":5379596522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] utils::geometry::circle::Circle2d::contains_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t3707541","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379596480,"raw_hex":"e995923800","size":5,"soft_fail":false,"target_va":5383304336,"va":5379596790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t3716564","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379596480,"raw_hex":"e9d4b53800","size":5,"soft_fail":false,"target_va":5383313216,"va":5379596647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":98468,"rva":10888080,"va":5379597200}
// 编译实例 RVAs: 0xa62390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t3716219","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379597200,"raw_hex":"e97bb43800","size":5,"soft_fail":false,"target_va":5383313440,"va":5379597216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 198
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":98004,"rva":10887296,"va":5379596416}
// 编译实例 RVAs: 0xa62080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::get_pos_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t3713227","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379596416,"raw_hex":"e9cba83800","size":5,"soft_fail":false,"target_va":5383309664,"va":5379596432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 205
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8143
// 首条位置: {"module":538,"record_offset":99528,"rva":10888416,"va":5379597536}
// 编译实例 RVAs: 0xa624e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t3725867","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379597536,"raw_hex":"e92bda3800","size":5,"soft_fail":false,"target_va":5383323424,"va":5379597552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos
// 记录形式: compiled_procedure; instances=1
// 源码行: 212
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":98624,"rva":10888112,"va":5379597232}
// 编译实例 RVAs: 0xa623b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t3719659","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379597232,"raw_hex":"e9ebc13800","size":5,"soft_fail":false,"target_va":5383316912,"va":5379597248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::expand
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=9
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x813E
// 首条位置: {"module":538,"record_offset":98936,"rva":10888160,"va":5379597280}
// 编译实例 RVAs: 0xa623e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::as_circle
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8140
// 首条位置: {"module":538,"record_offset":99084,"rva":10888224,"va":5379597344}
// 编译实例 RVAs: 0xa62420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37024851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379597344,"raw_hex":"e853f43402","size":5,"soft_fail":false,"target_va":5416622240,"va":5379597384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 241
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8136
// 首条位置: {"module":538,"record_offset":97852,"rva":10887248,"va":5379596368}
// 编译实例 RVAs: 0xa62050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37025827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379596368,"raw_hex":"e823f83402","size":5,"soft_fail":false,"target_va":5416622240,"va":5379596408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 269, 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97B1
// 首条位置: {"module":609,"inlinee":38833}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97CB
// 首条位置: {"module":1795,"inlinee":38859}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97CC
// 首条位置: {"module":1795,"inlinee":38860}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1795,"record_offset":16788,"parent_record_offset":16764,"ancestor_record_offset":16764,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1795,"record_offset":16812,"parent_record_offset":16788,"ancestor_record_offset":16764,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":16836,"parent_record_offset":16812,"ancestor_record_offset":16764,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8169
// 首条位置: {"module":609,"inlinee":33129}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155FC
// 首条位置: {"module":1795,"inlinee":87548}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1795,"record_offset":18476,"parent_record_offset":16052,"ancestor_record_offset":16052,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1795,"record_offset":19096,"parent_record_offset":16052,"ancestor_record_offset":16052,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":19160,"parent_record_offset":16052,"ancestor_record_offset":16052,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":16092,"parent_record_offset":16052,"ancestor_record_offset":16052,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":17884,"parent_record_offset":16052,"ancestor_record_offset":16052,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":18780,"parent_record_offset":18756,"ancestor_record_offset":16052,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":18828,"parent_record_offset":18804,"ancestor_record_offset":16052,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":18804,"parent_record_offset":18780,"ancestor_record_offset":16052,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":18756,"parent_record_offset":18732,"ancestor_record_offset":16052,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":18732,"parent_record_offset":18708,"ancestor_record_offset":16052,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":18708,"parent_record_offset":18676,"ancestor_record_offset":16052,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1795,"record_offset":16812,"parent_record_offset":16788,"ancestor_record_offset":16052,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1795,"record_offset":18328,"parent_record_offset":18200,"ancestor_record_offset":16052,"inlinee":7711,"source_line":1627,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":18260,"parent_record_offset":18232,"ancestor_record_offset":16052,"inlinee":12857,"source_line":795,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":18232,"parent_record_offset":18200,"ancestor_record_offset":16052,"inlinee":12856,"source_line":761,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":18288,"parent_record_offset":18260,"ancestor_record_offset":16052,"inlinee":12858,"source_line":1179,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1795,"record_offset":18996,"parent_record_offset":18972,"ancestor_record_offset":16052,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1795,"record_offset":16364,"parent_record_offset":16324,"ancestor_record_offset":16052,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":16836,"parent_record_offset":16812,"ancestor_record_offset":16052,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1795,"record_offset":18948,"parent_record_offset":18920,"ancestor_record_offset":16052,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1795,"record_offset":18972,"parent_record_offset":18948,"ancestor_record_offset":16052,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1795,"record_offset":16324,"parent_record_offset":16284,"ancestor_record_offset":16052,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1795,"record_offset":16788,"parent_record_offset":16764,"ancestor_record_offset":16052,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1795,"record_offset":17156,"parent_record_offset":17120,"ancestor_record_offset":16052,"inlinee":38859,"source_line":275,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1795,"record_offset":16764,"parent_record_offset":16724,"ancestor_record_offset":16052,"inlinee":38860,"source_line":275,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1795,"record_offset":16692,"parent_record_offset":16668,"ancestor_record_offset":16052,"inlinee":38816,"source_line":275,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1795,"record_offset":16476,"parent_record_offset":16416,"ancestor_record_offset":16052,"inlinee":87545,"source_line":275,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_serde.rs
//     evidence={"module":1795,"record_offset":18620,"parent_record_offset":18592,"ancestor_record_offset":16052,"inlinee":47685,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":18124,"parent_record_offset":18084,"ancestor_record_offset":16052,"inlinee":31975,"source_line":2023,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::MapRefDeserializer::next_pair
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":16284,"parent_record_offset":16212,"ancestor_record_offset":16052,"inlinee":11185,"source_line":2584,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::SeqRefDeserializer::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":19036,"parent_record_offset":18876,"ancestor_record_offset":16052,"inlinee":17577,"source_line":2482,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::SeqRefDeserializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":18920,"parent_record_offset":18876,"ancestor_record_offset":16052,"inlinee":17576,"source_line":2469,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":18084,"parent_record_offset":18044,"ancestor_record_offset":16052,"inlinee":31974,"source_line":2178,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":16536,"parent_record_offset":16476,"ancestor_record_offset":16052,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":18676,"parent_record_offset":18648,"ancestor_record_offset":16052,"inlinee":17629,"source_line":2310,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":18648,"parent_record_offset":18620,"ancestor_record_offset":16052,"inlinee":87550,"source_line":2327,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":16212,"parent_record_offset":16152,"ancestor_record_offset":16052,"inlinee":11183,"source_line":2642,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":17964,"parent_record_offset":17924,"ancestor_record_offset":16052,"inlinee":11186,"source_line":2655,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::visit_content_seq_ref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":18876,"parent_record_offset":18676,"ancestor_record_offset":16052,"inlinee":17574,"source_line":2043,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":16152,"parent_record_offset":16092,"ancestor_record_offset":16052,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":17924,"parent_record_offset":17884,"ancestor_record_offset":16052,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":17120,"parent_record_offset":16536,"ancestor_record_offset":16052,"inlinee":10116,"source_line":1602,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":16724,"parent_record_offset":16536,"ancestor_record_offset":16052,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":16668,"parent_record_offset":16536,"ancestor_record_offset":16052,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":16416,"parent_record_offset":16212,"ancestor_record_offset":16052,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":18044,"parent_record_offset":18004,"ancestor_record_offset":16052,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":18200,"parent_record_offset":18124,"ancestor_record_offset":16052,"inlinee":7710,"source_line":225,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":18420,"parent_record_offset":18124,"ancestor_record_offset":16052,"inlinee":17567,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":18392,"parent_record_offset":18124,"ancestor_record_offset":16052,"inlinee":17565,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":18360,"parent_record_offset":18124,"ancestor_record_offset":16052,"inlinee":7709,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155FD
// 首条位置: {"module":1795,"inlinee":87549}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":19364,"parent_record_offset":19312,"ancestor_record_offset":19312,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":19712,"parent_record_offset":19688,"ancestor_record_offset":19312,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":19760,"parent_record_offset":19736,"ancestor_record_offset":19312,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":19736,"parent_record_offset":19712,"ancestor_record_offset":19312,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":19688,"parent_record_offset":19664,"ancestor_record_offset":19312,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":19664,"parent_record_offset":19640,"ancestor_record_offset":19312,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":19640,"parent_record_offset":19608,"ancestor_record_offset":19312,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1795,"record_offset":20596,"parent_record_offset":20484,"ancestor_record_offset":19312,"inlinee":7711,"source_line":1627,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":20536,"parent_record_offset":20512,"ancestor_record_offset":19312,"inlinee":12857,"source_line":795,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":20512,"parent_record_offset":20484,"ancestor_record_offset":19312,"inlinee":12856,"source_line":761,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":20560,"parent_record_offset":20536,"ancestor_record_offset":19312,"inlinee":12858,"source_line":1179,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1795,"record_offset":19928,"parent_record_offset":19904,"ancestor_record_offset":19312,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1795,"record_offset":19492,"parent_record_offset":19468,"ancestor_record_offset":19312,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1795,"record_offset":20016,"parent_record_offset":19428,"ancestor_record_offset":19312,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1795,"record_offset":19880,"parent_record_offset":19852,"ancestor_record_offset":19312,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1795,"record_offset":19904,"parent_record_offset":19880,"ancestor_record_offset":19312,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1795,"record_offset":19468,"parent_record_offset":19428,"ancestor_record_offset":19312,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_serde.rs
//     evidence={"module":1795,"record_offset":19552,"parent_record_offset":19524,"ancestor_record_offset":19312,"inlinee":47685,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":20376,"parent_record_offset":20332,"ancestor_record_offset":19312,"inlinee":31975,"source_line":2023,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::SeqRefDeserializer::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":19968,"parent_record_offset":19808,"ancestor_record_offset":19312,"inlinee":17577,"source_line":2482,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::SeqRefDeserializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":19852,"parent_record_offset":19808,"ancestor_record_offset":19312,"inlinee":17576,"source_line":2469,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":20332,"parent_record_offset":20288,"ancestor_record_offset":19312,"inlinee":31974,"source_line":2178,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":19608,"parent_record_offset":19580,"ancestor_record_offset":19312,"inlinee":17629,"source_line":2310,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":19580,"parent_record_offset":19552,"ancestor_record_offset":19312,"inlinee":87550,"source_line":2327,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":19428,"parent_record_offset":19396,"ancestor_record_offset":19312,"inlinee":15023,"source_line":2527,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::visit_content_seq_ref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":19808,"parent_record_offset":19608,"ancestor_record_offset":19312,"inlinee":17574,"source_line":2043,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":19396,"parent_record_offset":19364,"ancestor_record_offset":19312,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":19524,"parent_record_offset":19428,"ancestor_record_offset":19312,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":20288,"parent_record_offset":20244,"ancestor_record_offset":19312,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":20484,"parent_record_offset":20376,"ancestor_record_offset":19312,"inlinee":7710,"source_line":225,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":20744,"parent_record_offset":20376,"ancestor_record_offset":19312,"inlinee":17567,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":20688,"parent_record_offset":20376,"ancestor_record_offset":19312,"inlinee":17565,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":20456,"parent_record_offset":20376,"ancestor_record_offset":19312,"inlinee":17563,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":20716,"parent_record_offset":20376,"ancestor_record_offset":19312,"inlinee":17566,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":20628,"parent_record_offset":20376,"ancestor_record_offset":19312,"inlinee":17564,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":20656,"parent_record_offset":20376,"ancestor_record_offset":19312,"inlinee":7709,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$2::from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 290
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x815F
// 首条位置: {"module":609,"inlinee":33119}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::wall::valid_enclosure::IsEnclosed
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed>", "constructor": "enum2$", "arguments": ["country_core::systems::wall::valid_enclosure::IsEnclosed"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> >", "constructor": "serde_json::ser::impl$10::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::wall::valid_enclosure::ShapeParameters
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 18; 过程引用数: 21
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>", "constructor": "enum2$", "arguments": ["country_core::systems::wall::valid_enclosure::ShapeParameters"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >", "constructor": "serde_json::ser::impl$10::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >", "constructor": "core::fmt::impl$80::fmt", "arguments": ["enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$9::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >", "constructor": "serde_json::value::ser::impl$9::serialize_field", "arguments": ["enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>,f32,f32>", "constructor": "tuple$", "arguments": ["enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>", "f32", "f32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tuple$<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>,f32,f32>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tuple$<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>,f32,f32>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 46; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] ref$<str$> (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)
// 候选参数类型: ["tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)
// 候选参数类型: ["rhapsody::raw::buffer::Buffer"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::as_circle
// 信息 ID: de299885a79a6756bd97cad7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x23b0f34 -> 0xa62420 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=574, inline_record=15660; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=574, inline_record=23460; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
// 信息 ID: 9ebb221b3d9222cc5f499e3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline::closure$1 @ 0xbdf363 -> 0xa62450 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141b90c -> 0xa62450 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x141e80d -> 0xa62450 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x141e903 -> 0xa62450 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::new @ 0x1490cce -> 0xa62450 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_glade_border::display_glade_border::display_glade_border @ 0x20a7767 -> 0xa62450 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x217002c -> 0xa62450 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x217089b -> 0xa62450 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x296d1fd -> 0xa62450 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::structure_outline::structure_outline::closure$1; module=583, inline_record=24508; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>; module=1720, inline_record=130956; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3
// 信息 ID: 956e52f64871183b30f1b4fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x883573 -> 0xa62490 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x883b8b -> 0xa62490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
// 信息 ID: dead64cb4c2b60db36590f34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x80464a -> 0xa62050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0x882841 -> 0xa62050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0x882af5 -> 0xa62050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::derive_decorator_subtype::is_half_dormer @ 0x8d896a -> 0xa62050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x8dd4c5 -> 0xa62050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x1439f28 -> 0xa62050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143c61d -> 0xa62050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143db03 -> 0xa62050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143e4e8 -> 0xa62050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143e4fb -> 0xa62050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1 @ 0x14d50c4 -> 0xa62050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id @ 0x14d560a -> 0xa62050 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0> @ 0x24f1921 -> 0xa62050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position @ 0x250ba71 -> 0xa62050 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims; module=567, inline_record=44788; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::center
// 信息 ID: d2a51031981b277a7802efea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::plaster_systems::maintain_plaster @ 0x809f11 -> 0xa623d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_floors::construct_floors @ 0x80ea0d -> 0xa623d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::create_roof::create_roof @ 0x28270dc -> 0xa623d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_beam_count_change_audio::timberframe_beam_count_change_audio @ 0x28345a5 -> 0xa623d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0x2939b5e -> 0xa623d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group; module=510, inline_record=71412; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::WallGroupMember::target_state; module=561, inline_record=21888; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::WallGroupMember::new; module=561, inline_record=22896; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::stone_foundation_offset::set_wall_stone_foundation_offset; module=575, inline_record=40192; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::structure_outline::structure_outline::closure$1; module=583, inline_record=70236; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::pivot_3d; module=584, inline_record=25872; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>; module=1720, inline_record=130468; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::systems::nani_update_stone_floor; module=3273, inline_record=104924; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::contains_point
// 信息 ID: a188797a5324366fa60e5202
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85bbb2 -> 0xa620a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x863027 -> 0xa620a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x868154 -> 0xa620a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_categorize::circle_categorize @ 0x1481a90 -> 0xa620a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x14826c6 -> 0xa620a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0x148359b -> 0xa620a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x14869f5 -> 0xa620a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0x148813c -> 0xa620a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x148b30d -> 0xa620a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x148dc41 -> 0xa620a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::ui_place_tree::ui_place_tree @ 0x2860967 -> 0xa620a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::ui_move_tree::ui_move_tree @ 0x28631c9 -> 0xa620a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0x2939b3d -> 0xa620a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings @ 0x293b975 -> 0xa620a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings @ 0x293c5c6 -> 0xa620a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::ui_draw_wall::ui_draw_wall; module=503, inline_record=88196; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::ui_draw_wall::ui_draw_wall; module=503, inline_record=90696; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::ui_draw_wall::ui_draw_wall; module=503, inline_record=90916; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_pruner::ivy_pruner; module=538, inline_record=35336; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_pruner::ivy_pruner; module=538, inline_record=35372; 不是独立调用边。
// [PDB 内联宿主] core::slice::iter::impl$171::all<country_core::resources::walls::inner_wall_state::WallState,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$3>; module=561, inline_record=89912; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::glade_border::GladeBorder::is_pos_inside; module=567, inline_record=76604; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::glade_border::GladeBorder::is_curve2_inside; module=567, inline_record=78112; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape
// 信息 ID: 6e3bce4800727b372c1d14d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x21ba0dd -> 0xa620c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x21ba247 -> 0xa620c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::expand
// 信息 ID: b0e253a7bde90b3c759b09e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs @ 0x886c1e -> 0xa623e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::new @ 0x1490cc1 -> 0xa623e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x14e0d57 -> 0xa623e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x14e0df0 -> 0xa623e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e305b -> 0xa623e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e30d0 -> 0xa623e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x2170011 -> 0xa623e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x2170872 -> 0xa623e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x21ba034 -> 0xa623e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x21ba05b -> 0xa623e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x21ba0b3 -> 0xa623e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x21ba21b -> 0xa623e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x21c28dd -> 0xa623e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x21c28f5 -> 0xa623e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0x2939af1 -> 0xa623e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line; module=557, inline_record=22464; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::structure_outline::structure_outline::closure$1; module=583, inline_record=22300; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::structure_outline::structure_outline::closure$1; module=583, inline_record=36900; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::structure_outline::structure_outline::closure$1; module=583, inline_record=64412; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::public_wall_state::PublicWallState::premade_wooden_wall_outer_shape; module=585, inline_record=93552; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos
// 信息 ID: 0eea840d812b3b27af2de1e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x8834f0 -> 0xa623b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x14e0708 -> 0xa623b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x14e0729 -> 0xa623b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x14e0e09 -> 0xa623b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x14e0e2a -> 0xa623b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e2b93 -> 0xa623b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e2c6e -> 0xa623b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e30e9 -> 0xa623b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e310a -> 0xa623b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x21c3045 -> 0xa623b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x21c3069 -> 0xa623b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u
// 信息 ID: c5579bdf09b18c181410d7a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143d520 -> 0xa62390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143ca19 -> 0xa62390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0x148806c -> 0xa62390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x148b244 -> 0xa62390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0> @ 0x24e2a56 -> 0xa62390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u
// 信息 ID: 06d60092f9b0390aadb8bb60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 18 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<system_wall_constructor::construct_wooden_walls::construct_wall_boards::WallSample,core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_wooden_walls::construct_wall_boards::impl$2::new::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::WallSample,alloc::vec::impl$21::extend_trusted::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::WallSample,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_wooden_walls::construct_wall_boards::impl$2::new::closure_env$0> > > > @ 0x142ca7b -> 0xa62080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x14e073e -> 0xa62080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x14e0757 -> 0xa62080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x14e0f9a -> 0xa62080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x14e1014 -> 0xa62080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x14e108d -> 0xa62080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x14e1101 -> 0xa62080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e2baa -> 0xa62080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e2c82 -> 0xa62080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e32f9 -> 0xa62080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e3356 -> 0xa62080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e3403 -> 0xa62080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e3460 -> 0xa62080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e3620 -> 0xa62080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e3692 -> 0xa62080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e36cd -> 0xa62080 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
// 信息 ID: 8caabc10d5b11205c4bd2ec5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 20 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation @ 0x7fc1b0 -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x883522 -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x883b3a -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline::closure$1 @ 0xbdf3cc -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1 @ 0x14d513b -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id @ 0x14d5669 -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x14e050e -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e2e24 -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e311b -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e312b -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_glade_border::display_glade_border::display_glade_border @ 0x20a78a3 -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x21ba0f2 -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x21ba129 -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x21ba25c -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x21ba293 -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x21c308f -> 0xa624e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::structure_outline::structure_outline::closure$1; module=583, inline_record=25320; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
// 信息 ID: bdf9006fc8d903f823c8c4dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=609, inline_record=6400; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=609, inline_record=6668; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=609, inline_record=7364; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::private::de::missing_field::MissingFieldDeserializer<serde_json::error::Error> >; module=609, inline_record=8520; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::private::de::content::ContentRefDeserializer<serde_json::error::Error> >; module=609, inline_record=11352; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::private::de::content::ContentRefDeserializer<serde_json::error::Error> >; module=609, inline_record=11480; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::private::de::content::ContentRefDeserializer<serde_json::error::Error> >; module=609, inline_record=12172; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=609, inline_record=13204; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 3a26c8d3ee8eab3dea4d8487
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__FieldVisitor>; module=609, inline_record=57396; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__FieldVisitor>; module=609, inline_record=57624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes
// 信息 ID: 442bf6867364d475a97f491c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=17156; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=17184; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=17212; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=17244; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=17468; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=17692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>
// 信息 ID: 418d7ce6a5e929739a085012
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 005142dbe05efbcf5874b4d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=16764; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=16876; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=16988; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=17300; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=17524; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=17748; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 543d506d89ae6afa2d194195
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: 2820be90717703ed5c78c654
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 14 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1795, inline_record=2032; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1795, inline_record=2228; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1795, inline_record=5196; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1795, inline_record=5392; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1795, inline_record=6272; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1795, inline_record=6468; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1795, inline_record=7352; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1795, inline_record=7548; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 63a6d3667b1da30d47c02a78
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1795, inline_record=1924; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1795, inline_record=5088; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1795, inline_record=6168; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1795, inline_record=7244; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=16476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 459f6119b1d8b88369b2e928
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=609, inline_record=57844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 29ec1f3b6df339ca38d0f162
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=15044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes
// 信息 ID: aac7a9cb98dcaab60fd44a35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=74948; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=74980; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=77808; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=77840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: cfb17fa794403efc6fdeb376
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=74764; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=75008; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=77624; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=77868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
// 信息 ID: d6b4210501b6528df05f46bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=74656; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=74704; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=77516; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=77564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: 65e70b282225847e26d2f6e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=74560; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=77420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: 067b875ec1655e60efb8d8ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=74240; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=77100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 信息 ID: c027f3269de57da2610bfebc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=75872; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1795, inline_record=78732; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum<serde::private::de::content::EnumRefDeserializer<serde_json::error::Error> >
// 信息 ID: d0796927da8c63e9cc58e4fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde::private::de::content::EnumRefDeserializer -> ["serde_json::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum -> ["serde::private::de::content::EnumRefDeserializer<serde_json::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor> @ 0x1b4e353 -> 0xc88df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor> @ 0x1b4e2b3 -> 0xc88ea0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor> @ 0x1b4e213 -> 0xc89230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 3d29747887df05970f15717a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=16052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 3d7d2fffa70c7d569ed4dd79
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=19312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 41266d3d22b09f0f69ed9ae4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::visit_array<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor> @ 0x1b9caa2 -> 0xc85c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::visit_array<country_core::resources::roofs::roof_shape::_::impl$0::deserialize::__Visitor> @ 0x1b9dc13 -> 0xc864f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::private::de::content::ContentRefDeserializer<serde_json::error::Error> >
// 信息 ID: b180363c7822e09cdcc6afbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde::private::de::content::ContentRefDeserializer -> ["serde_json::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize -> ["serde::private::de::content::ContentRefDeserializer<serde_json::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1b577ad -> 0xc86190 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1b577eb -> 0xc86190 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1b57b0d -> 0xc86190 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1b57b4b -> 0xc86190 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::private::de::missing_field::MissingFieldDeserializer<serde_json::error::Error> >
// 信息 ID: 8a4d66df21b50151a7a62d5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde::private::de::missing_field::MissingFieldDeserializer -> ["serde_json::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize -> ["serde::private::de::missing_field::MissingFieldDeserializer<serde_json::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1b5788d -> 0x2d7b5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$52::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1b57bed -> 0x2d7b5a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::serialize
// 信息 ID: a405957279fa56c7ff7b4791
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::ser::impl$9::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >; module=1802, inline_record=3256; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >; module=1802, inline_record=80636; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> >; module=1802, inline_record=221480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 8c3794aa5c45f89a5119b10d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> > @ 0x465002 -> 0x5a34e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> > @ 0x4641a2 -> 0x5a3750 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> > @ 0x18bbe02 -> 0xc85750 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> > @ 0x18b97f2 -> 0xc85f80 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 426744d2aa33372c1dcf4fa2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> > @ 0x1b896cc -> 0xc859c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$0::from
// 信息 ID: 20d67d7a063138dcec509719
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=609, inline_record=8128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$10::eq
// 信息 ID: 8ce16da62e9f6ea237d0583b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::ui_move_wall_group; module=561, inline_record=32116; 不是独立调用边。
// [PDB 内联宿主] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners; module=2741, inline_record=13772; 不是独立调用边。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3779, inline_record=4500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$2::from
// 信息 ID: c8c5ffb50a14359e7a68055f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::private::de::content::ContentRefDeserializer<serde_json::error::Error> >; module=609, inline_record=12944; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=609, inline_record=14940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$3::clone
// 信息 ID: 193e8f6df042ae810424e047
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c5c60f15599d089557c43d58: enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > (*)(country_core::systems::ui_move_wall_group::WallGroupMember)
// 引用方: 原程序直接转移 0 条；内联宿主 37 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::cache_if_original_wall; module=520, inline_record=24176; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=40108; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=40988; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=46468; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=48272; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=50172; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall; module=535, inline_record=39108; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall; module=535, inline_record=40540; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$4::fmt
// 信息 ID: 8fefe5c926bc275a2b47bde4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$6::clone
// 信息 ID: b91ce7c76d33e903ded96a63
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs; module=452, inline_record=20972; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::cloned::impl$1::next<bevy_ecs::query::iter::QueryIter<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,country_core::resources::roofs::roof_shape::Roof>; module=594, inline_record=3472; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::component::clone::component_clone_via_clone<country_core::resources::roofs::roof_shape::Roof>; module=1636, inline_record=151964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$7::fmt
// 信息 ID: 1d1c599dc14429227f29d239
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >; module=609, inline_record=57148; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$80::fmt<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >; module=4664, inline_record=448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
