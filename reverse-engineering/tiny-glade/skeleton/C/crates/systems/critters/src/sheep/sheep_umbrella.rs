// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/critters/src/sheep/sheep_umbrella.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: c60cb183ed0b50f0587479197584995fb42ce2149b1cf013d8b5b5e40008e125
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sheep_umbrella.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 4171
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_umbrella::sheep_umbrellas
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4171,"record_offset":272,"rva":43239552,"va":5411948672}
// 编译实例 RVAs: 0x293c880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-36272231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411948672,"raw_hex":"e89987d6fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5411950930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b45a0
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: sheep_umbrella.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42503688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411948672,"raw_hex":"e8f87177fd","size":5,"soft_fail":false,"target_va":5369447840,"va":5411951523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409ab250
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sheep_umbrella.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33103455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411948672,"raw_hex":"e8a1e106fe","size":5,"soft_fail":false,"target_va":5378847312,"va":5411950762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7b7d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sheep_umbrella.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32250061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411948672,"raw_hex":"e833e713fe","size":5,"soft_fail":false,"target_va":5379700688,"va":5411950744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14294d290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sheep_umbrella.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t66661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411948672,"raw_hex":"e865040100","size":5,"soft_fail":false,"target_va":5412016784,"va":5411950118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sheep_umbrella.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4401868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411948672,"raw_hex":"e8cc2a4300","size":5,"soft_fail":false,"target_va":5416353040,"va":5411951167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27327752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411948672,"raw_hex":"e8f8025ffe","size":5,"soft_fail":false,"target_va":5384624080,"va":5411951827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4670357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411948672,"raw_hex":"e895434700","size":5,"soft_fail":false,"target_va":5416621600,"va":5411951238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4670335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411948672,"raw_hex":"e87f434700","size":5,"soft_fail":false,"target_va":5416621632,"va":5411951292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32258908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411948672,"raw_hex":"e8a4c413fe","size":5,"soft_fail":false,"target_va":5379691776,"va":5411950679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioInstance::start_if_not_playing
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32268662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411948672,"raw_hex":"e88a9e13fe","size":5,"soft_fail":false,"target_va":5379682096,"va":5411950753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-28653844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411948672,"raw_hex":"e8ecc64afe","size":5,"soft_fail":false,"target_va":5383297200,"va":5411951039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4171,"record_offset":10412,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4171,"record_offset":888,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4171,"record_offset":10080,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4171,"record_offset":5700,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11799,"source_line":1524,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4171,"record_offset":452,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4171,"record_offset":11824,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4171,"record_offset":10468,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4171,"record_offset":10692,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4171,"record_offset":10616,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_direct] country_core::components::drawable::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":4171,"record_offset":7816,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":81034,"source_line":145,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4171,"record_offset":11856,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4171,"record_offset":10896,"parent_record_offset":10872,"ancestor_record_offset":272,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4171,"record_offset":10920,"parent_record_offset":10896,"ancestor_record_offset":272,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4171,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":272,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4171,"record_offset":10872,"parent_record_offset":10816,"ancestor_record_offset":272,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4171,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4171,"record_offset":10232,"parent_record_offset":10204,"ancestor_record_offset":272,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4171,"record_offset":10316,"parent_record_offset":10292,"ancestor_record_offset":272,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4171,"record_offset":10844,"parent_record_offset":10816,"ancestor_record_offset":272,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4171,"record_offset":10816,"parent_record_offset":10792,"ancestor_record_offset":272,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4171,"record_offset":10292,"parent_record_offset":10260,"ancestor_record_offset":272,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4171,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4171,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4171,"record_offset":10260,"parent_record_offset":10232,"ancestor_record_offset":272,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4171,"record_offset":10792,"parent_record_offset":10768,"ancestor_record_offset":272,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4171,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":272,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4171,"record_offset":672,"parent_record_offset":640,"ancestor_record_offset":272,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4171,"record_offset":1192,"parent_record_offset":1168,"ancestor_record_offset":272,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4171,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":272,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4171,"record_offset":10204,"parent_record_offset":10168,"ancestor_record_offset":272,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4171,"record_offset":640,"parent_record_offset":612,"ancestor_record_offset":272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4171,"record_offset":4788,"parent_record_offset":912,"ancestor_record_offset":272,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4171,"record_offset":1400,"parent_record_offset":912,"ancestor_record_offset":272,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4171,"record_offset":3040,"parent_record_offset":912,"ancestor_record_offset":272,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4171,"record_offset":5040,"parent_record_offset":912,"ancestor_record_offset":272,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4171,"record_offset":6124,"parent_record_offset":5732,"ancestor_record_offset":272,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4171,"record_offset":1316,"parent_record_offset":912,"ancestor_record_offset":272,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4171,"record_offset":10436,"parent_record_offset":10412,"ancestor_record_offset":272,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4171,"record_offset":5784,"parent_record_offset":5732,"ancestor_record_offset":272,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4171,"record_offset":5812,"parent_record_offset":5784,"ancestor_record_offset":272,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entity::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4171,"record_offset":5844,"parent_record_offset":5812,"ancestor_record_offset":272,"inlinee":8684,"source_line":588,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::EntityRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4171,"record_offset":5880,"parent_record_offset":5844,"ancestor_record_offset":272,"inlinee":8686,"source_line":140,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4171,"record_offset":6028,"parent_record_offset":5996,"ancestor_record_offset":272,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4171,"record_offset":6052,"parent_record_offset":6028,"ancestor_record_offset":272,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4171,"record_offset":5116,"parent_record_offset":5092,"ancestor_record_offset":272,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4171,"record_offset":3164,"parent_record_offset":3132,"ancestor_record_offset":272,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4171,"record_offset":1516,"parent_record_offset":1484,"ancestor_record_offset":272,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4171,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":272,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4171,"record_offset":5140,"parent_record_offset":5116,"ancestor_record_offset":272,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4171,"record_offset":4288,"parent_record_offset":3132,"ancestor_record_offset":272,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4171,"record_offset":2576,"parent_record_offset":1484,"ancestor_record_offset":272,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4171,"record_offset":7524,"parent_record_offset":7500,"ancestor_record_offset":272,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4171,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":272,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4171,"record_offset":7500,"parent_record_offset":5732,"ancestor_record_offset":272,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$92::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4171,"record_offset":5068,"parent_record_offset":912,"ancestor_record_offset":272,"inlinee":11825,"source_line":2520,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4171,"record_offset":588,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4171,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":272,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4171,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":272,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4171,"record_offset":6544,"parent_record_offset":5732,"ancestor_record_offset":272,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4171,"record_offset":3132,"parent_record_offset":912,"ancestor_record_offset":272,"inlinee":10710,"source_line":182,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4171,"record_offset":1484,"parent_record_offset":912,"ancestor_record_offset":272,"inlinee":10708,"source_line":195,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":4171,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":272,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4171,"record_offset":1616,"parent_record_offset":1592,"ancestor_record_offset":272,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4171,"record_offset":1592,"parent_record_offset":1568,"ancestor_record_offset":272,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4171,"record_offset":1796,"parent_record_offset":1772,"ancestor_record_offset":272,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4171,"record_offset":1568,"parent_record_offset":1540,"ancestor_record_offset":272,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4171,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":272,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4171,"record_offset":1168,"parent_record_offset":912,"ancestor_record_offset":272,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4171,"record_offset":5164,"parent_record_offset":5140,"ancestor_record_offset":272,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4171,"record_offset":1084,"parent_record_offset":912,"ancestor_record_offset":272,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4171,"record_offset":2976,"parent_record_offset":2576,"ancestor_record_offset":272,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4171,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":272,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4171,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":272,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":4171,"record_offset":3000,"parent_record_offset":2976,"ancestor_record_offset":272,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4171,"record_offset":10140,"parent_record_offset":10112,"ancestor_record_offset":272,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4171,"record_offset":10168,"parent_record_offset":10140,"ancestor_record_offset":272,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4171,"record_offset":10112,"parent_record_offset":10080,"ancestor_record_offset":272,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::entity_command::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/entity_command.rs
//     evidence={"module":4171,"record_offset":10380,"parent_record_offset":10080,"ancestor_record_offset":272,"inlinee":98163,"source_line":108,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4171,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":272,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4171,"record_offset":5732,"parent_record_offset":5700,"ancestor_record_offset":272,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4171,"record_offset":552,"parent_record_offset":528,"ancestor_record_offset":272,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4171,"record_offset":476,"parent_record_offset":452,"ancestor_record_offset":272,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4171,"record_offset":504,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":4171,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":272,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":4171,"record_offset":6160,"parent_record_offset":6124,"ancestor_record_offset":272,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4171,"record_offset":8136,"parent_record_offset":8108,"ancestor_record_offset":272,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4171,"record_offset":5928,"parent_record_offset":5904,"ancestor_record_offset":272,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4171,"record_offset":11880,"parent_record_offset":11856,"ancestor_record_offset":272,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4171,"record_offset":1708,"parent_record_offset":1644,"ancestor_record_offset":272,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4171,"record_offset":2704,"parent_record_offset":2680,"ancestor_record_offset":272,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4171,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":272,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4171,"record_offset":5996,"parent_record_offset":5812,"ancestor_record_offset":272,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4171,"record_offset":1772,"parent_record_offset":1592,"ancestor_record_offset":272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4171,"record_offset":6096,"parent_record_offset":5732,"ancestor_record_offset":272,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4171,"record_offset":8300,"parent_record_offset":7936,"ancestor_record_offset":272,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4171,"record_offset":10660,"parent_record_offset":10616,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4171,"record_offset":836,"parent_record_offset":812,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4171,"record_offset":8108,"parent_record_offset":8084,"ancestor_record_offset":272,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4171,"record_offset":8164,"parent_record_offset":8108,"ancestor_record_offset":272,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4171,"record_offset":8188,"parent_record_offset":8164,"ancestor_record_offset":272,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4171,"record_offset":8060,"parent_record_offset":8036,"ancestor_record_offset":272,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4171,"record_offset":8084,"parent_record_offset":8060,"ancestor_record_offset":272,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4171,"record_offset":1048,"parent_record_offset":1020,"ancestor_record_offset":272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4171,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":272,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4171,"record_offset":4984,"parent_record_offset":912,"ancestor_record_offset":272,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4171,"record_offset":1132,"parent_record_offset":1108,"ancestor_record_offset":272,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4171,"record_offset":1852,"parent_record_offset":1820,"ancestor_record_offset":272,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4171,"record_offset":812,"parent_record_offset":640,"ancestor_record_offset":272,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4171,"record_offset":1020,"parent_record_offset":912,"ancestor_record_offset":272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4171,"record_offset":8236,"parent_record_offset":8008,"ancestor_record_offset":272,"inlinee":12347,"source_line":3134,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4171,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":272,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":4171,"record_offset":7880,"parent_record_offset":7852,"ancestor_record_offset":272,"inlinee":8011,"source_line":81,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::shader_library::render_pipelines_impl::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/shader_library.rs
//     evidence={"module":4171,"record_offset":7912,"parent_record_offset":7816,"ancestor_record_offset":272,"inlinee":32867,"source_line":61,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::drawable::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":4171,"record_offset":7852,"parent_record_offset":7816,"ancestor_record_offset":272,"inlinee":81036,"source_line":115,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::drawable::impl$22::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":4171,"record_offset":9700,"parent_record_offset":7816,"ancestor_record_offset":272,"inlinee":34697,"source_line":154,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::drawable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":4171,"record_offset":7936,"parent_record_offset":7912,"ancestor_record_offset":272,"inlinee":32855,"source_line":17,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4171,"record_offset":6208,"parent_record_offset":6184,"ancestor_record_offset":272,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4171,"record_offset":6232,"parent_record_offset":6208,"ancestor_record_offset":272,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4171,"record_offset":6184,"parent_record_offset":6160,"ancestor_record_offset":272,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4171,"record_offset":5188,"parent_record_offset":5164,"ancestor_record_offset":272,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4171,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":272,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::binding_set::impl$26::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":4171,"record_offset":9372,"parent_record_offset":9344,"ancestor_record_offset":272,"inlinee":16899,"source_line":78,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::binding_set::impl$43::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":4171,"record_offset":9752,"parent_record_offset":9724,"ancestor_record_offset":272,"inlinee":30000,"source_line":78,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":4171,"record_offset":8036,"parent_record_offset":8008,"ancestor_record_offset":272,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$24::clone
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":4171,"record_offset":8008,"parent_record_offset":7984,"ancestor_record_offset":272,"inlinee":12345,"source_line":766,"archive":"_pdb/C/modules/04171.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::impl$44::clone
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":4171,"record_offset":7984,"parent_record_offset":7936,"ancestor_record_offset":272,"inlinee":32857,"source_line":78,"archive":"_pdb/C/modules/04171.jsonl.gz"}
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
// 完整结构及原始类型记录: sheep_umbrella.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sheep_umbrella.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sheep_umbrella.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_umbrella::sheep_umbrellas
// 信息 ID: 92798e2bcf0aa3b03027896b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3b47c9e896ee223b6ab00e3f: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x22a5daf -> 0x293c880 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x22b53d1 -> 0x293c880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
