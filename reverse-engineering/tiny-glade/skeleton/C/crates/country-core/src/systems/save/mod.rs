// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/save/mod.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: a4372c11e159bf66a5c587172830bac8cbd5e837f4c2dd87f024c1f6c5e4ae24
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 43, 210, 234, 506, 511, 535, 564, 579, 1636, 1700
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7126
// 首条位置: {"module":564,"inlinee":28966}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":564,"record_offset":9372,"parent_record_offset":9316,"ancestor_record_offset":9316,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impls::impl$55::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":564,"record_offset":9344,"parent_record_offset":9316,"ancestor_record_offset":9316,"inlinee":9964,"source_line":132,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x222A
// 首条位置: {"module":43,"inlinee":8746}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::ser::impls::impl$42::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":43,"record_offset":46480,"parent_record_offset":46456,"ancestor_record_offset":46456,"inlinee":8748,"source_line":11,"archive":"_pdb/C/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":43,"record_offset":46552,"parent_record_offset":46528,"ancestor_record_offset":46456,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$28::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/number.rs
//     evidence={"module":43,"record_offset":46576,"parent_record_offset":46552,"ancestor_record_offset":46456,"inlinee":7655,"source_line":768,"archive":"_pdb/C/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_i32
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":43,"record_offset":46504,"parent_record_offset":46480,"ancestor_record_offset":46456,"inlinee":8749,"source_line":88,"archive":"_pdb/C/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_i64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":43,"record_offset":46528,"parent_record_offset":46504,"ancestor_record_offset":46456,"inlinee":8750,"source_line":92,"archive":"_pdb/C/modules/00043.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::impl$1::add_save_load_systems
// 记录形式: compiled_procedure; instances=1
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":511,"record_offset":157952,"rva":9724992,"va":5378434112}
// 编译实例 RVAs: 0x946440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..boxed..Box$LT$dyn$u20$bevy_ecs..system..system..System$u2b$In$u20$$u3d$$u20$$LP$$RP$$u2b$Out$u20$$u3d$$u20$$LP$$RP$$GT$$u20$as$u20$bevy_ecs..schedule..config..Schedulable$GT$::into_config
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t5784576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e800445800","size":5,"soft_fail":false,"target_va":5384218864,"va":5378434283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-2755550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e822f4d5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378434233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9712031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e861ce6bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378434218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400793a0
//     owner=unresolved_namespace; count=8
//     target_aliases=118 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9230112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e8e02873ff","size":5,"soft_fail":false,"target_va":5369205664,"va":5378435771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140090990
//     owner=unresolved_namespace; count=1
//     target_aliases=31 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9134333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e8039f74ff","size":5,"soft_fail":false,"target_va":5369301392,"va":5378435720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd210
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7836448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e8e06c88ff","size":5,"soft_fail":false,"target_va":5370597904,"va":5378434347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140956560
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t65603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e843000100","size":5,"soft_fail":false,"target_va":5378499936,"va":5378434328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098bc50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t283123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e8f3510400","size":5,"soft_fail":false,"target_va":5378718800,"va":5378435672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14151b060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12404347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e87b46bd00","size":5,"soft_fail":false,"target_va":5390839904,"va":5378435552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417e48e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15328223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e8dfe3e900","size":5,"soft_fail":false,"target_va":5393762528,"va":5378434300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417e4ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15329497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e8d9e8e900","size":5,"soft_fail":false,"target_va":5393764048,"va":5378434546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417e50b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15330344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e828ece900","size":5,"soft_fail":false,"target_va":5393764528,"va":5378434179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417e5480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15330742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e8b6ede900","size":5,"soft_fail":false,"target_va":5393765504,"va":5378434757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417e5660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15330941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e87deee900","size":5,"soft_fail":false,"target_va":5393765984,"va":5378435038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419db770
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17388282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e8fa520901","size":5,"soft_fail":false,"target_va":5395822448,"va":5378434161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38622788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e844564d02","size":5,"soft_fail":false,"target_va":5417057056,"va":5378434263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38185182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378434112,"raw_hex":"e8dea84602","size":5,"soft_fail":false,"target_va":5416620769,"va":5378435582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":511,"record_offset":159512,"parent_record_offset":157952,"ancestor_record_offset":157952,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":511,"record_offset":158104,"parent_record_offset":157952,"ancestor_record_offset":157952,"inlinee":4638,"source_line":448,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":511,"record_offset":158404,"parent_record_offset":157952,"ancestor_record_offset":157952,"inlinee":5917,"source_line":493,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":158300,"parent_record_offset":158276,"ancestor_record_offset":157952,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":158324,"parent_record_offset":158300,"ancestor_record_offset":157952,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":158348,"parent_record_offset":158324,"ancestor_record_offset":157952,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":158276,"parent_record_offset":158244,"ancestor_record_offset":157952,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":511,"record_offset":158244,"parent_record_offset":158204,"ancestor_record_offset":157952,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":511,"record_offset":158204,"parent_record_offset":158136,"ancestor_record_offset":157952,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::impl$4::into_configs
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":511,"record_offset":158136,"parent_record_offset":158104,"ancestor_record_offset":157952,"inlinee":4259,"source_line":565,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/label.rs
//     evidence={"module":511,"record_offset":158440,"parent_record_offset":158404,"ancestor_record_offset":157952,"inlinee":4539,"source_line":119,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::function_system::impl$5::into_system
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs
//     evidence={"module":511,"record_offset":158176,"parent_record_offset":158136,"ancestor_record_offset":157952,"inlinee":5856,"source_line":595,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::impl$1::add_save_load_events_and_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":511,"record_offset":159544,"rva":9727008,"va":5378436128}
// 编译实例 RVAs: 0x946c20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6207096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e878b65e00","size":5,"soft_fail":false,"target_va":5384644912,"va":5378437811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::thread::builder::Builder>::name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6212449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e861cb5e00","size":5,"soft_fail":false,"target_va":5384649296,"va":5378436842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-2757640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e8f8ebd5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378436323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2761587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e88ddcd5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378440286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=6
//     target_aliases=1699 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9714121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e837c66bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378436308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007db70
//     owner=unresolved_namespace; count=2
//     target_aliases=27 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9215815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e8b96073ff","size":5,"soft_fail":false,"target_va":5369224048,"va":5378439858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008f880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9142645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e88b7e74ff","size":5,"soft_fail":false,"target_va":5369297024,"va":5378439664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9102245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e85b1c75ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5378441088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b6e30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8981904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e870f276ff","size":5,"soft_fail":false,"target_va":5369458224,"va":5378440123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096e150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t156184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e818620200","size":5,"soft_fail":false,"target_va":5378597200,"va":5378441011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096edf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t159434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e8ca6e0200","size":5,"soft_fail":false,"target_va":5378600432,"va":5378440993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140971f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t172308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e814a10200","size":5,"soft_fail":false,"target_va":5378613120,"va":5378440807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409730f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t176790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e896b20200","size":5,"soft_fail":false,"target_va":5378617584,"va":5378440789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097d3e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t219721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e8495a0300","size":5,"soft_fail":false,"target_va":5378659296,"va":5378439570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a9b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t401682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e812210600","size":5,"soft_fail":false,"target_va":5378841456,"va":5378439769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b2afc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1979490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e862341e00","size":5,"soft_fail":false,"target_va":5380419520,"va":5378440025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90900
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3443678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e8de8b3400","size":5,"soft_fail":false,"target_va":5381884160,"va":5378440477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141824fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15589400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e818e0ed00","size":5,"soft_fail":false,"target_va":5394026400,"va":5378436995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141865450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15851567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e82fe0f100","size":5,"soft_fail":false,"target_va":5394289744,"va":5378438172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141869cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15869753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e83927f200","size":5,"soft_fail":false,"target_va":5394308272,"va":5378438514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141870270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15895986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e8b28df200","size":5,"soft_fail":false,"target_va":5394334320,"va":5378438329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38620695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e8174e4d02","size":5,"soft_fail":false,"target_va":5417057056,"va":5378436356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t38181469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e85d9a4602","size":5,"soft_fail":false,"target_va":5416620769,"va":5378439295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38181335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e8d7994602","size":5,"soft_fail":false,"target_va":5416620794,"va":5378439454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::message::message_registry::MessageRegistry::register_message
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15718063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e8afd6ef00","size":5,"soft_fail":false,"target_va":5394155360,"va":5378437292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38182120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e8e89c4602","size":5,"soft_fail":false,"target_va":5416621632,"va":5378439507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6287321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378436128,"raw_hex":"e8d9ef5f00","size":5,"soft_fail":false,"target_va":5384726688,"va":5378439362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":511,"record_offset":160780,"parent_record_offset":159544,"ancestor_record_offset":159544,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":511,"record_offset":160664,"parent_record_offset":159544,"ancestor_record_offset":159544,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$62::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":511,"record_offset":160384,"parent_record_offset":159544,"ancestor_record_offset":159544,"inlinee":7477,"source_line":3726,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_message
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":511,"record_offset":161456,"parent_record_offset":159544,"ancestor_record_offset":159544,"inlinee":11549,"source_line":400,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":173468,"parent_record_offset":159544,"ancestor_record_offset":159544,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":511,"record_offset":161416,"parent_record_offset":159544,"ancestor_record_offset":159544,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::load_game::LoadGameManager::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/load_game.rs
//     evidence={"module":511,"record_offset":168232,"parent_record_offset":159544,"ancestor_record_offset":159544,"inlinee":29498,"source_line":82,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::steam::missing_mods::impl$10::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/missing_mods.rs
//     evidence={"module":511,"record_offset":166460,"parent_record_offset":159544,"ancestor_record_offset":159544,"inlinee":29496,"source_line":97,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::channel
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":511,"record_offset":159740,"parent_record_offset":159544,"ancestor_record_offset":159544,"inlinee":7548,"source_line":519,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] std::thread::builder::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":511,"record_offset":160752,"parent_record_offset":159544,"ancestor_record_offset":159544,"inlinee":7549,"source_line":78,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] std::thread::builder::Builder::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":511,"record_offset":161324,"parent_record_offset":159544,"ancestor_record_offset":159544,"inlinee":7491,"source_line":185,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":159952,"parent_record_offset":159928,"ancestor_record_offset":159544,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":159976,"parent_record_offset":159952,"ancestor_record_offset":159544,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":173664,"parent_record_offset":173640,"ancestor_record_offset":159544,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":173688,"parent_record_offset":173664,"ancestor_record_offset":159544,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":160000,"parent_record_offset":159976,"ancestor_record_offset":159544,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":173712,"parent_record_offset":173688,"ancestor_record_offset":159544,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":159928,"parent_record_offset":159896,"ancestor_record_offset":159544,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":173640,"parent_record_offset":173612,"ancestor_record_offset":159544,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":511,"record_offset":159896,"parent_record_offset":159848,"ancestor_record_offset":159544,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":511,"record_offset":159848,"parent_record_offset":159812,"ancestor_record_offset":159544,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":511,"record_offset":160420,"parent_record_offset":160384,"ancestor_record_offset":159544,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$6::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":511,"record_offset":160604,"parent_record_offset":160384,"ancestor_record_offset":159544,"inlinee":7479,"source_line":1212,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":162852,"parent_record_offset":162828,"ancestor_record_offset":159544,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":161012,"parent_record_offset":160976,"ancestor_record_offset":159544,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":173612,"parent_record_offset":173588,"ancestor_record_offset":159544,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":162900,"parent_record_offset":162876,"ancestor_record_offset":159544,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":162876,"parent_record_offset":162852,"ancestor_record_offset":159544,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":161088,"parent_record_offset":161048,"ancestor_record_offset":159544,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":161048,"parent_record_offset":161012,"ancestor_record_offset":159544,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":173588,"parent_record_offset":173564,"ancestor_record_offset":159544,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":511,"record_offset":160864,"parent_record_offset":160816,"ancestor_record_offset":159544,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":511,"record_offset":160900,"parent_record_offset":160864,"ancestor_record_offset":159544,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":511,"record_offset":160936,"parent_record_offset":160900,"ancestor_record_offset":159544,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":511,"record_offset":160816,"parent_record_offset":160780,"ancestor_record_offset":159544,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":511,"record_offset":173992,"parent_record_offset":173956,"ancestor_record_offset":159544,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":511,"record_offset":173804,"parent_record_offset":173780,"ancestor_record_offset":159544,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$62::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":511,"record_offset":167948,"parent_record_offset":166460,"ancestor_record_offset":159544,"inlinee":7477,"source_line":3726,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":162828,"parent_record_offset":162804,"ancestor_record_offset":159544,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":162804,"parent_record_offset":162764,"ancestor_record_offset":159544,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":166548,"parent_record_offset":166524,"ancestor_record_offset":159544,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":160976,"parent_record_offset":160936,"ancestor_record_offset":159544,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$27::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":166524,"parent_record_offset":166460,"ancestor_record_offset":159544,"inlinee":7206,"source_line":4236,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":162764,"parent_record_offset":162740,"ancestor_record_offset":159544,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_message
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs
//     evidence={"module":511,"record_offset":161480,"parent_record_offset":161456,"ancestor_record_offset":159544,"inlinee":9392,"source_line":348,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":511,"record_offset":161540,"parent_record_offset":161508,"ancestor_record_offset":159544,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":511,"record_offset":163072,"parent_record_offset":163048,"ancestor_record_offset":159544,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":511,"record_offset":163048,"parent_record_offset":163024,"ancestor_record_offset":159544,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":511,"record_offset":162512,"parent_record_offset":162488,"ancestor_record_offset":159544,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":162560,"parent_record_offset":162536,"ancestor_record_offset":159544,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":162536,"parent_record_offset":162512,"ancestor_record_offset":159544,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":162740,"parent_record_offset":162716,"ancestor_record_offset":159544,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":511,"record_offset":161508,"parent_record_offset":161480,"ancestor_record_offset":159544,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":511,"record_offset":162488,"parent_record_offset":162464,"ancestor_record_offset":159544,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":511,"record_offset":161564,"parent_record_offset":161540,"ancestor_record_offset":159544,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":511,"record_offset":162212,"parent_record_offset":162188,"ancestor_record_offset":159544,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":511,"record_offset":162240,"parent_record_offset":162212,"ancestor_record_offset":159544,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":511,"record_offset":167120,"parent_record_offset":167092,"ancestor_record_offset":159544,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":511,"record_offset":167020,"parent_record_offset":166816,"ancestor_record_offset":159544,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":511,"record_offset":161780,"parent_record_offset":161756,"ancestor_record_offset":159544,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":511,"record_offset":161864,"parent_record_offset":161840,"ancestor_record_offset":159544,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":511,"record_offset":160628,"parent_record_offset":160604,"ancestor_record_offset":159544,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":511,"record_offset":166984,"parent_record_offset":166960,"ancestor_record_offset":159544,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":511,"record_offset":167148,"parent_record_offset":167092,"ancestor_record_offset":159544,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":511,"record_offset":162012,"parent_record_offset":161984,"ancestor_record_offset":159544,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":511,"record_offset":161984,"parent_record_offset":161956,"ancestor_record_offset":159544,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":511,"record_offset":162652,"parent_record_offset":162588,"ancestor_record_offset":159544,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":162464,"parent_record_offset":161508,"ancestor_record_offset":159544,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":162676,"parent_record_offset":162652,"ancestor_record_offset":159544,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":163000,"parent_record_offset":161508,"ancestor_record_offset":159544,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":162716,"parent_record_offset":162536,"ancestor_record_offset":159544,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":166848,"parent_record_offset":166816,"ancestor_record_offset":159544,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":511,"record_offset":161248,"parent_record_offset":160936,"ancestor_record_offset":159544,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":161272,"parent_record_offset":161248,"ancestor_record_offset":159544,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":173492,"parent_record_offset":173468,"ancestor_record_offset":159544,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":511,"record_offset":166936,"parent_record_offset":166816,"ancestor_record_offset":159544,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":511,"record_offset":162128,"parent_record_offset":162104,"ancestor_record_offset":159544,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":511,"record_offset":174024,"parent_record_offset":173992,"ancestor_record_offset":159544,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":166960,"parent_record_offset":166936,"ancestor_record_offset":159544,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":511,"record_offset":161376,"parent_record_offset":161348,"ancestor_record_offset":159544,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":511,"record_offset":162588,"parent_record_offset":162560,"ancestor_record_offset":159544,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":511,"record_offset":162624,"parent_record_offset":162588,"ancestor_record_offset":159544,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":511,"record_offset":162944,"parent_record_offset":162764,"ancestor_record_offset":159544,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":511,"record_offset":160692,"parent_record_offset":160664,"ancestor_record_offset":159544,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":511,"record_offset":173840,"parent_record_offset":173804,"ancestor_record_offset":159544,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":511,"record_offset":160716,"parent_record_offset":160692,"ancestor_record_offset":159544,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":511,"record_offset":173864,"parent_record_offset":173840,"ancestor_record_offset":159544,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":511,"record_offset":173896,"parent_record_offset":173804,"ancestor_record_offset":159544,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":511,"record_offset":162188,"parent_record_offset":162164,"ancestor_record_offset":159544,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":511,"record_offset":161924,"parent_record_offset":161896,"ancestor_record_offset":159544,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":511,"record_offset":161956,"parent_record_offset":161924,"ancestor_record_offset":159544,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":511,"record_offset":161896,"parent_record_offset":161676,"ancestor_record_offset":159544,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":511,"record_offset":161756,"parent_record_offset":161676,"ancestor_record_offset":159544,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":511,"record_offset":162316,"parent_record_offset":161676,"ancestor_record_offset":159544,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":511,"record_offset":161840,"parent_record_offset":161676,"ancestor_record_offset":159544,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":511,"record_offset":161600,"parent_record_offset":161564,"ancestor_record_offset":159544,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":511,"record_offset":162164,"parent_record_offset":162056,"ancestor_record_offset":159544,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":162104,"parent_record_offset":162080,"ancestor_record_offset":159544,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":162408,"parent_record_offset":161676,"ancestor_record_offset":159544,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":162080,"parent_record_offset":162056,"ancestor_record_offset":159544,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":161652,"parent_record_offset":161628,"ancestor_record_offset":159544,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":161628,"parent_record_offset":161600,"ancestor_record_offset":159544,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":161676,"parent_record_offset":161652,"ancestor_record_offset":159544,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":161728,"parent_record_offset":161676,"ancestor_record_offset":159544,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":162056,"parent_record_offset":161676,"ancestor_record_offset":159544,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":511,"record_offset":162288,"parent_record_offset":161676,"ancestor_record_offset":159544,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":511,"record_offset":166580,"parent_record_offset":166460,"ancestor_record_offset":159544,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":511,"record_offset":166636,"parent_record_offset":166608,"ancestor_record_offset":159544,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":511,"record_offset":166896,"parent_record_offset":166872,"ancestor_record_offset":159544,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":511,"record_offset":166756,"parent_record_offset":166728,"ancestor_record_offset":159544,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":511,"record_offset":167092,"parent_record_offset":166692,"ancestor_record_offset":159544,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":511,"record_offset":166608,"parent_record_offset":166580,"ancestor_record_offset":159544,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::channel
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\mod.rs
//     evidence={"module":511,"record_offset":159776,"parent_record_offset":159740,"ancestor_record_offset":159544,"inlinee":5896,"source_line":198,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::counter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\counter.rs
//     evidence={"module":511,"record_offset":159812,"parent_record_offset":159776,"ancestor_record_offset":159544,"inlinee":5885,"source_line":20,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":511,"record_offset":166784,"parent_record_offset":166756,"ancestor_record_offset":159544,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":511,"record_offset":166816,"parent_record_offset":166784,"ancestor_record_offset":159544,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":511,"record_offset":161348,"parent_record_offset":161324,"ancestor_record_offset":159544,"inlinee":7492,"source_line":254,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":511,"record_offset":166692,"parent_record_offset":166664,"ancestor_record_offset":159544,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":511,"record_offset":166664,"parent_record_offset":166636,"ancestor_record_offset":159544,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::impl$1::add_save_load_events_and_resources::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7F6F
// 首条位置: {"module":535,"inlinee":32623}
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
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::SaveFileVersion
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
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
// 文件级候选接口: 64; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)
// 候选参数类型: ["tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >", "tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::generate_clothline::CacheClothes>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::system::commands::Commands", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::generate_clothline::CacheClothes>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::generate_clothline::CacheClothes>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::decorator::DecoratorOpEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::decorator::DecoratorOpEvent>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::decorator::DecoratorOpEvent>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::decorator::DecoratorOpEvent>,bevy_ecs::message::message_reader::MessageReader<system_decorator::audio::op_preview_audio::MutePreviewDecoratorOpSounds>,bevy_ecs::change_detection::ResMut<enum2$<system_decorator::audio::op_preview_audio::PreviousFrameDecoratorPreviewOp> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>,bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::decorator::DecoratorOpEvent>", "bevy_ecs::message::message_reader::MessageReader<system_decorator::audio::op_preview_audio::MutePreviewDecoratorOpSounds>", "bevy_ecs::change_detection::ResMut<enum2$<system_decorator::audio::op_preview_audio::PreviousFrameDecoratorPreviewOp> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>", "bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::decorator::DecoratorOpEvent>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_decorator::audio::op_preview_audio::MutePreviewDecoratorOpSounds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<system_decorator::audio::op_preview_audio::PreviousFrameDecoratorPreviewOp> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::system::system_param::Local<system_decorator::audio::op_stairs_audio::PrevFrameStairStats>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::decorator::DecoratorOpEvent>,bevy_ecs::message::message_reader::MessageReader<system_decorator::audio::op_preview_audio::MutePreviewDecoratorOpSounds>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>", "bevy_ecs::system::system_param::Local<system_decorator::audio::op_stairs_audio::PrevFrameStairStats>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::decorator::DecoratorOpEvent>", "bevy_ecs::message::message_reader::MessageReader<system_decorator::audio::op_preview_audio::MutePreviewDecoratorOpSounds>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::audio::op_stairs_audio::PrevFrameStairStats>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::decorator::DecoratorOpEvent>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_decorator::audio::op_preview_audio::MutePreviewDecoratorOpSounds>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::_::impl$0::deserialize
// 信息 ID: 968b17f286d0ccafa9cce390
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::load_game::parse_saved_game_data; module=564, inline_record=9316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::_::impl$0::serialize
// 信息 ID: f8fa7e2bf6bf196685ba79e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game_to_file<std::fs::File,core::slice::iter::Iter<country_core::resources::history::MultiEdit> >; module=43, inline_record=46456; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game_to_file<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> >,core::slice::iter::Iter<country_core::resources::history::MultiEdit> >; module=1700, inline_record=140528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::impl$1::add_save_load_events_and_resources
// 信息 ID: 594cd2c9956e390cf1963fae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x41e1a9 -> 0x946c20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::impl$1::add_save_load_events_and_resources::closure$0
// 信息 ID: 22dca50d8758e3529eff2fd0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::impl$1::add_save_load_events_and_resources", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0 <- core::ptr::drop_in_place<std::thread::lifecycle::spawn_unchecked::MaybeDangling<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0 <- std::sys::backtrace::__rust_begin_short_backtrace<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0 <- std::thread::lifecycle::spawn_unchecked<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::lifecycle::spawn_unchecked::closure_env$1<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0 <- core::ptr::drop_in_place<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0 <- core::ptr::drop_in_place<std::thread::lifecycle::spawn_unchecked::closure_env$1<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0,tuple$<> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0,tuple$<> >; module=535, inline_record=113056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::impl$1::add_save_load_systems
// 信息 ID: 561c0197c39593f8736bcc55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x41e161 -> 0x946440 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
