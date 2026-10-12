// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/tree-tool/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 33e2906d851b6fd837ab91f08bd3eff6d5a98e9db084b80297ec6233bc699437
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 313, 2600, 3887, 3900
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_tree::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":313,"record_offset":1520,"rva":7257888,"va":5375967008}
// 编译实例 RVAs: 0x6ebf20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8677603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375967008,"raw_hex":"e8e3688400","size":5,"soft_fail":false,"target_va":5384644912,"va":5375967304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400793a0
//     owner=unresolved_namespace; count=2
//     target_aliases=118 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6763740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375967008,"raw_hex":"e824cb98ff","size":5,"soft_fail":false,"target_va":5369205664,"va":5375969399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5369224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375967008,"raw_hex":"e87812aeff","size":5,"soft_fail":false,"target_va":5370597904,"va":5375967123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10045685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375967008,"raw_hex":"e8f5489900","size":5,"soft_fail":false,"target_va":5386013088,"va":5375967398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10033270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375967008,"raw_hex":"e876189900","size":5,"soft_fail":false,"target_va":5386000320,"va":5375967045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10043352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375967008,"raw_hex":"e8d83f9900","size":5,"soft_fail":false,"target_va":5386010464,"va":5375967107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::message::message_registry::MessageRegistry::register_message
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10048726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375967008,"raw_hex":"e8d6549900","size":5,"soft_fail":false,"target_va":5386016368,"va":5375967637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10040698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375967008,"raw_hex":"e87a359900","size":5,"soft_fail":false,"target_va":5386007776,"va":5375967073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::in_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10036548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375967008,"raw_hex":"e844259900","size":5,"soft_fail":false,"target_va":5386003616,"va":5375967063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8757346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375967008,"raw_hex":"e862a08500","size":5,"soft_fail":false,"target_va":5384726688,"va":5375969337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_message
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":313,"record_offset":2488,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":11549,"source_line":400,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":313,"record_offset":1644,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":313,"record_offset":1672,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":5917,"source_line":493,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::tree_storage::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/tree_storage.rs
//     evidence={"module":313,"record_offset":1768,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20441,"source_line":51,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":313,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":1520,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":313,"record_offset":3932,"parent_record_offset":3908,"ancestor_record_offset":1520,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":313,"record_offset":3908,"parent_record_offset":3884,"ancestor_record_offset":1520,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":313,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":1520,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":313,"record_offset":3836,"parent_record_offset":3796,"ancestor_record_offset":1520,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":313,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":1520,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_message
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs
//     evidence={"module":313,"record_offset":2512,"parent_record_offset":2488,"ancestor_record_offset":1520,"inlinee":9392,"source_line":348,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":313,"record_offset":2572,"parent_record_offset":2540,"ancestor_record_offset":1520,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/label.rs
//     evidence={"module":313,"record_offset":1708,"parent_record_offset":1672,"ancestor_record_offset":1520,"inlinee":4539,"source_line":119,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":313,"record_offset":4104,"parent_record_offset":4080,"ancestor_record_offset":1520,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":313,"record_offset":4080,"parent_record_offset":4056,"ancestor_record_offset":1520,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":313,"record_offset":3544,"parent_record_offset":3520,"ancestor_record_offset":1520,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":313,"record_offset":3592,"parent_record_offset":3568,"ancestor_record_offset":1520,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":313,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":1520,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":313,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":1520,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":313,"record_offset":2540,"parent_record_offset":2512,"ancestor_record_offset":1520,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":313,"record_offset":3520,"parent_record_offset":3496,"ancestor_record_offset":1520,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":313,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":1520,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":313,"record_offset":3244,"parent_record_offset":3220,"ancestor_record_offset":1520,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":313,"record_offset":3272,"parent_record_offset":3244,"ancestor_record_offset":1520,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":313,"record_offset":2340,"parent_record_offset":2312,"ancestor_record_offset":1520,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":313,"record_offset":2240,"parent_record_offset":2036,"ancestor_record_offset":1520,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":313,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":1520,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":313,"record_offset":2896,"parent_record_offset":2872,"ancestor_record_offset":1520,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":313,"record_offset":2204,"parent_record_offset":2180,"ancestor_record_offset":1520,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":313,"record_offset":2368,"parent_record_offset":2312,"ancestor_record_offset":1520,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":313,"record_offset":3044,"parent_record_offset":3016,"ancestor_record_offset":1520,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":313,"record_offset":3016,"parent_record_offset":2988,"ancestor_record_offset":1520,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":313,"record_offset":1948,"parent_record_offset":1916,"ancestor_record_offset":1520,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":313,"record_offset":3496,"parent_record_offset":2540,"ancestor_record_offset":1520,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":313,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":1520,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":313,"record_offset":4032,"parent_record_offset":2540,"ancestor_record_offset":1520,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":313,"record_offset":3748,"parent_record_offset":3568,"ancestor_record_offset":1520,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":313,"record_offset":2068,"parent_record_offset":2036,"ancestor_record_offset":1520,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":313,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":1520,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":313,"record_offset":2156,"parent_record_offset":2036,"ancestor_record_offset":1520,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":313,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":1520,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":313,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":1520,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":313,"record_offset":3620,"parent_record_offset":3592,"ancestor_record_offset":1520,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":313,"record_offset":3656,"parent_record_offset":3620,"ancestor_record_offset":1520,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":313,"record_offset":3976,"parent_record_offset":3796,"ancestor_record_offset":1520,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":313,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":1520,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":313,"record_offset":2956,"parent_record_offset":2928,"ancestor_record_offset":1520,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":313,"record_offset":2988,"parent_record_offset":2956,"ancestor_record_offset":1520,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":313,"record_offset":2928,"parent_record_offset":2708,"ancestor_record_offset":1520,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":313,"record_offset":2788,"parent_record_offset":2708,"ancestor_record_offset":1520,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":313,"record_offset":3348,"parent_record_offset":2708,"ancestor_record_offset":1520,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":313,"record_offset":2872,"parent_record_offset":2708,"ancestor_record_offset":1520,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":313,"record_offset":2632,"parent_record_offset":2596,"ancestor_record_offset":1520,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":313,"record_offset":3196,"parent_record_offset":3088,"ancestor_record_offset":1520,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":313,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":1520,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":313,"record_offset":3440,"parent_record_offset":2708,"ancestor_record_offset":1520,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":313,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":1520,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":313,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":1520,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":313,"record_offset":2660,"parent_record_offset":2632,"ancestor_record_offset":1520,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":313,"record_offset":2708,"parent_record_offset":2684,"ancestor_record_offset":1520,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":313,"record_offset":2760,"parent_record_offset":2708,"ancestor_record_offset":1520,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":313,"record_offset":3088,"parent_record_offset":2708,"ancestor_record_offset":1520,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":313,"record_offset":3320,"parent_record_offset":2708,"ancestor_record_offset":1520,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":313,"record_offset":1804,"parent_record_offset":1768,"ancestor_record_offset":1520,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":313,"record_offset":1860,"parent_record_offset":1832,"ancestor_record_offset":1520,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":313,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":1520,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":313,"record_offset":1976,"parent_record_offset":1948,"ancestor_record_offset":1520,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":313,"record_offset":2312,"parent_record_offset":1916,"ancestor_record_offset":1520,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":313,"record_offset":1832,"parent_record_offset":1804,"ancestor_record_offset":1520,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":313,"record_offset":2004,"parent_record_offset":1976,"ancestor_record_offset":1520,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":313,"record_offset":2036,"parent_record_offset":2004,"ancestor_record_offset":1520,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":313,"record_offset":1916,"parent_record_offset":1888,"ancestor_record_offset":1520,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00313.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":313,"record_offset":1888,"parent_record_offset":1860,"ancestor_record_offset":1520,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00313.jsonl.gz"}
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
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_tree::TreePlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<system_tree::TreePlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["system_tree::TreePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<system_tree::TreePlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["system_tree::TreePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<system_tree::TreePlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["system_tree::TreePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<system_tree::TreePlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["system_tree::TreePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<system_tree::TreePlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["system_tree::TreePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<system_tree::TreePlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["system_tree::TreePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<system_tree::TreePlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["system_tree::TreePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<system_tree::TreePlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["system_tree::TreePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$15::add_to_app<bevy_app::plugin::sealed::PluginMarker,feedback_reporter::FeedbackReporterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::menu_background::MenuBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::paths::PathPlugin,bevy_app::plugin::sealed::PluginMarker,system_water::WaterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::garden::GardenPlugin,bevy_app::plugin::sealed::PluginMarker,system_tree::TreePlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::prefab::PrefabPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest,bevy_app::plugin::sealed::PluginMarker,country_core::resources::photo_mode::PhotoModePlugin>", "constructor": "bevy_app::plugin::sealed::impl$15::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "feedback_reporter::FeedbackReporterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::menu_background::MenuBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::paths::PathPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_water::WaterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::garden::GardenPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_tree::TreePlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::prefab::PrefabPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::vermeer::VermeerPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::photo_mode::PhotoModePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 函数: system_tree::impl$0::build
// 信息 ID: 400ae02501b37c1258f4f18f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
