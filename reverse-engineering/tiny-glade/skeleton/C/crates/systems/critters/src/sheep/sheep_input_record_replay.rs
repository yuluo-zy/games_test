// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/critters/src/sheep/sheep_input_record_replay.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1944fe7e02a6ffc32d180f91dd433beb75ffdb03f09e6692f3e2ce3499962a8b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sheep_input_record_replay.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2858, 4167
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4167,"record_offset":272,"rva":43231296,"va":5411940416}
// 编译实例 RVAs: 0x293a840
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$glam..f32..sse2..quat..Quat$u20$as$u20$glam..euler..FromEuler$GT$::from_euler_angles
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-27329995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411940416,"raw_hex":"e835fa5efe","size":5,"soft_fail":false,"target_va":5384611280,"va":5411941270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-36262788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411940416,"raw_hex":"e87cacd6fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5411941487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a58f10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sheep_input_record_replay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32381382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411940416,"raw_hex":"e83ae611fe","size":5,"soft_fail":false,"target_va":5379559184,"va":5411940561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422bdae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sheep_input_record_replay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6803807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411940416,"raw_hex":"e8a12e98ff","size":5,"soft_fail":false,"target_va":5405137632,"va":5411941434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422be5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sheep_input_record_replay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6801211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411940416,"raw_hex":"e8c53898ff","size":5,"soft_fail":false,"target_va":5405140400,"va":5411941606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4679892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411940416,"raw_hex":"e8d4684700","size":5,"soft_fail":false,"target_va":5416621600,"va":5411941703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::input_stream::InputStream::last_frame_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32381704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411940416,"raw_hex":"e8f8e411fe","size":5,"soft_fail":false,"target_va":5379559744,"va":5411941443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4167,"record_offset":468,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4167,"record_offset":4184,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4167,"record_offset":4324,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":10312,"source_line":673,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4167,"record_offset":528,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":4167,"record_offset":1244,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4167,"record_offset":4796,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4167,"record_offset":964,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4167,"record_offset":4880,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4167,"record_offset":4916,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_euler
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":4167,"record_offset":4240,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":32144,"source_line":194,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4167,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":272,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4167,"record_offset":5128,"parent_record_offset":5104,"ancestor_record_offset":272,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4167,"record_offset":5152,"parent_record_offset":5128,"ancestor_record_offset":272,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4167,"record_offset":5080,"parent_record_offset":4988,"ancestor_record_offset":272,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4167,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4167,"record_offset":5016,"parent_record_offset":4988,"ancestor_record_offset":272,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4167,"record_offset":4988,"parent_record_offset":4964,"ancestor_record_offset":272,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4167,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4167,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4167,"record_offset":4964,"parent_record_offset":4940,"ancestor_record_offset":272,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4167,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":272,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4167,"record_offset":748,"parent_record_offset":716,"ancestor_record_offset":272,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4167,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":272,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4167,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":272,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4167,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":272,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4167,"record_offset":988,"parent_record_offset":964,"ancestor_record_offset":272,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4167,"record_offset":716,"parent_record_offset":688,"ancestor_record_offset":272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4167,"record_offset":3436,"parent_record_offset":1440,"ancestor_record_offset":272,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4167,"record_offset":2740,"parent_record_offset":1440,"ancestor_record_offset":272,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4167,"record_offset":2824,"parent_record_offset":1440,"ancestor_record_offset":272,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4167,"record_offset":3688,"parent_record_offset":1440,"ancestor_record_offset":272,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4167,"record_offset":2656,"parent_record_offset":1440,"ancestor_record_offset":272,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4167,"record_offset":496,"parent_record_offset":468,"ancestor_record_offset":272,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4167,"record_offset":4208,"parent_record_offset":4184,"ancestor_record_offset":272,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4167,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":272,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4167,"record_offset":2916,"parent_record_offset":1440,"ancestor_record_offset":272,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4167,"record_offset":1808,"parent_record_offset":1440,"ancestor_record_offset":272,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4167,"record_offset":2300,"parent_record_offset":1440,"ancestor_record_offset":272,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4167,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":272,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4167,"record_offset":664,"parent_record_offset":552,"ancestor_record_offset":272,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4167,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":272,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4167,"record_offset":1440,"parent_record_offset":1408,"ancestor_record_offset":272,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4167,"record_offset":1408,"parent_record_offset":1372,"ancestor_record_offset":272,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":4167,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":272,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4167,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":272,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4167,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":272,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4167,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":272,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4167,"record_offset":1840,"parent_record_offset":1808,"ancestor_record_offset":272,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4167,"record_offset":1724,"parent_record_offset":1440,"ancestor_record_offset":272,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4167,"record_offset":3800,"parent_record_offset":3764,"ancestor_record_offset":272,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4167,"record_offset":1640,"parent_record_offset":1440,"ancestor_record_offset":272,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_added_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4167,"record_offset":2184,"parent_record_offset":1808,"ancestor_record_offset":272,"inlinee":29006,"source_line":305,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4167,"record_offset":2240,"parent_record_offset":1808,"ancestor_record_offset":272,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4167,"record_offset":2128,"parent_record_offset":1808,"ancestor_record_offset":272,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":4167,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":272,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4167,"record_offset":4348,"parent_record_offset":4324,"ancestor_record_offset":272,"inlinee":11849,"source_line":545,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4167,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":272,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4167,"record_offset":628,"parent_record_offset":604,"ancestor_record_offset":272,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4167,"record_offset":552,"parent_record_offset":528,"ancestor_record_offset":272,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4167,"record_offset":580,"parent_record_offset":552,"ancestor_record_offset":272,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4167,"record_offset":5232,"parent_record_offset":5208,"ancestor_record_offset":272,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":4167,"record_offset":4488,"parent_record_offset":4460,"ancestor_record_offset":272,"inlinee":11874,"source_line":264,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":4167,"record_offset":2384,"parent_record_offset":2348,"ancestor_record_offset":272,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":4167,"record_offset":1276,"parent_record_offset":1244,"ancestor_record_offset":272,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":4167,"record_offset":1308,"parent_record_offset":1276,"ancestor_record_offset":272,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":4167,"record_offset":4844,"parent_record_offset":4820,"ancestor_record_offset":272,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4167,"record_offset":5048,"parent_record_offset":5016,"ancestor_record_offset":272,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4167,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":272,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4167,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":272,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4167,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":272,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4167,"record_offset":2000,"parent_record_offset":1864,"ancestor_record_offset":272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4167,"record_offset":4024,"parent_record_offset":1372,"ancestor_record_offset":272,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4167,"record_offset":4940,"parent_record_offset":4916,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4167,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4167,"record_offset":1608,"parent_record_offset":1548,"ancestor_record_offset":272,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4167,"record_offset":1580,"parent_record_offset":1548,"ancestor_record_offset":272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4167,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":272,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4167,"record_offset":3632,"parent_record_offset":1440,"ancestor_record_offset":272,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4167,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":272,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4167,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":272,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4167,"record_offset":2080,"parent_record_offset":2048,"ancestor_record_offset":272,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4167,"record_offset":888,"parent_record_offset":716,"ancestor_record_offset":272,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4167,"record_offset":1548,"parent_record_offset":1440,"ancestor_record_offset":272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4167,"record_offset":3832,"parent_record_offset":3800,"ancestor_record_offset":272,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4167,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":272,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/04167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B320
// 首条位置: {"module":2858,"inlinee":111392}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B31F
// 首条位置: {"module":2858,"inlinee":111391}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::to_euler
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":2858,"record_offset":17900,"parent_record_offset":17876,"ancestor_record_offset":17876,"inlinee":32217,"source_line":500,"archive":"_pdb/C/modules/02858.jsonl.gz"}
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
// 完整结构及原始类型记录: sheep_input_record_replay.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sheep_input_record_replay.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sheep_input_record_replay.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay
// 信息 ID: 031574a45596059247c882c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1fcca224b2574abbeaac05ae: void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),tuple$<>,void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> > @ 0x22a622d -> 0x293a840 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),tuple$<>,void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> @ 0x22b62c9 -> 0x293a840 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure$0
// 信息 ID: f9c7d60eb5ef55311a348fab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay", "captures": "unknown"}
// [闭包环境类型引用] system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0 <- core::iter::adapters::map::impl$2::next<glam::f32::vec3::Vec3,bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::next<glam::f32::vec3::Vec3,bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0>; module=2858, inline_record=21068; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0> >; module=2858, inline_record=51956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure$1
// 信息 ID: 855a0e1172b0a452b95312b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay", "captures": "unknown"}
// [闭包环境类型引用] system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1 <- core::iter::adapters::map::impl$2::next<f32,bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::next<f32,bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1>; module=2858, inline_record=17876; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1> >; module=2858, inline_record=66060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
