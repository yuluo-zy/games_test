// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/shader_refactoring.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: aeaef1c569956ee32ac7fbe7d77ae2854510b78e5dfd780de27f0a452fa28610
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shader_refactoring.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 522
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::shader_refactoring::render_shader_refactoring
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":522,"record_offset":58544,"rva":10025456,"va":5378734576}
// 编译实例 RVAs: 0x98f9f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1021976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e8e867f0ff","size":5,"soft_fail":false,"target_va":5377716144,"va":5378738115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408edd00
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-664116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e8ccddf5ff","size":5,"soft_fail":false,"target_va":5378071808,"va":5378735919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee240
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-662701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e853e3f5ff","size":5,"soft_fail":false,"target_va":5378073152,"va":5378735848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096c890
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-144492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e894cbfdff","size":5,"soft_fail":false,"target_va":5378590864,"va":5378735351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t316472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e838d40400","size":5,"soft_fail":false,"target_va":5379052752,"va":5378736275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2387616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e8a06e2400","size":5,"soft_fail":false,"target_va":5381123296,"va":5378735675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2434051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e803242500","size":5,"soft_fail":false,"target_va":5381170352,"va":5378736296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13054953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e8e933c700","size":5,"soft_fail":false,"target_va":5391790752,"va":5378735794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33111298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e8023df901","size":5,"soft_fail":false,"target_va":5411847168,"va":5378735865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3916491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e8cbc23b00","size":5,"soft_fail":false,"target_va":5382655024,"va":5378738528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32803963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e87b8cf401","size":5,"soft_fail":false,"target_va":5411541776,"va":5378737808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32804067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e8e38cf401","size":5,"soft_fail":false,"target_va":5411542064,"va":5378737992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32804962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e86290f401","size":5,"soft_fail":false,"target_va":5411542880,"va":5378737913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2382582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e8f65a2400","size":5,"soft_fail":false,"target_va":5381117840,"va":5378735253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2381286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e8e6552400","size":5,"soft_fail":false,"target_va":5381116576,"va":5378735285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2381461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e895562400","size":5,"soft_fail":false,"target_va":5381116784,"va":5378735318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2381511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e8c7562400","size":5,"soft_fail":false,"target_va":5381116816,"va":5378735300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2463336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e868962500","size":5,"soft_fail":false,"target_va":5381199056,"va":5378735715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::read_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32801750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e8d683f401","size":5,"soft_fail":false,"target_va":5411539504,"va":5378737749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32802463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378734576,"raw_hex":"e89f86f401","size":5,"soft_fail":false,"target_va":5411540288,"va":5378737820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":522,"record_offset":62576,"parent_record_offset":58544,"ancestor_record_offset":58544,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":522,"record_offset":58760,"parent_record_offset":58544,"ancestor_record_offset":58544,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":522,"record_offset":64260,"parent_record_offset":58544,"ancestor_record_offset":58544,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":522,"record_offset":62500,"parent_record_offset":58544,"ancestor_record_offset":58544,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":522,"record_offset":62956,"parent_record_offset":58544,"ancestor_record_offset":58544,"inlinee":26886,"source_line":452,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":522,"record_offset":62908,"parent_record_offset":58544,"ancestor_record_offset":58544,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":522,"record_offset":62860,"parent_record_offset":58544,"ancestor_record_offset":58544,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":522,"record_offset":62732,"parent_record_offset":58544,"ancestor_record_offset":58544,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":522,"record_offset":62472,"parent_record_offset":58544,"ancestor_record_offset":58544,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":522,"record_offset":62008,"parent_record_offset":61984,"ancestor_record_offset":58544,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":522,"record_offset":62056,"parent_record_offset":62032,"ancestor_record_offset":58544,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":522,"record_offset":62032,"parent_record_offset":62008,"ancestor_record_offset":58544,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":522,"record_offset":61984,"parent_record_offset":61960,"ancestor_record_offset":58544,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":522,"record_offset":61960,"parent_record_offset":61920,"ancestor_record_offset":58544,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":522,"record_offset":61920,"parent_record_offset":61896,"ancestor_record_offset":58544,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":522,"record_offset":62612,"parent_record_offset":62576,"ancestor_record_offset":58544,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":522,"record_offset":58816,"parent_record_offset":58760,"ancestor_record_offset":58544,"inlinee":8098,"source_line":2472,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_resource_id::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":522,"record_offset":59812,"parent_record_offset":59772,"ancestor_record_offset":58544,"inlinee":8100,"source_line":2473,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_resource_id::closure$0::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":522,"record_offset":61196,"parent_record_offset":61172,"ancestor_record_offset":58544,"inlinee":8103,"source_line":2479,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":522,"record_offset":62308,"parent_record_offset":62284,"ancestor_record_offset":58544,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":522,"record_offset":62284,"parent_record_offset":62260,"ancestor_record_offset":58544,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":522,"record_offset":62232,"parent_record_offset":62200,"ancestor_record_offset":58544,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":522,"record_offset":62176,"parent_record_offset":61560,"ancestor_record_offset":58544,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":522,"record_offset":62200,"parent_record_offset":62176,"ancestor_record_offset":58544,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":522,"record_offset":61604,"parent_record_offset":61560,"ancestor_record_offset":58544,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":522,"record_offset":62260,"parent_record_offset":62200,"ancestor_record_offset":58544,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":522,"record_offset":61676,"parent_record_offset":61640,"ancestor_record_offset":58544,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":522,"record_offset":61640,"parent_record_offset":61604,"ancestor_record_offset":58544,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":522,"record_offset":61896,"parent_record_offset":61872,"ancestor_record_offset":58544,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":522,"record_offset":62376,"parent_record_offset":62352,"ancestor_record_offset":58544,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":522,"record_offset":62352,"parent_record_offset":61560,"ancestor_record_offset":58544,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":522,"record_offset":61560,"parent_record_offset":58760,"ancestor_record_offset":58544,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":522,"record_offset":58860,"parent_record_offset":58816,"ancestor_record_offset":58544,"inlinee":5614,"source_line":933,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":522,"record_offset":59652,"parent_record_offset":59628,"ancestor_record_offset":58544,"inlinee":5068,"source_line":719,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":522,"record_offset":59700,"parent_record_offset":59676,"ancestor_record_offset":58544,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":522,"record_offset":62800,"parent_record_offset":62732,"ancestor_record_offset":58544,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":522,"record_offset":59100,"parent_record_offset":59076,"ancestor_record_offset":58544,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":522,"record_offset":59184,"parent_record_offset":59160,"ancestor_record_offset":58544,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":522,"record_offset":59124,"parent_record_offset":59100,"ancestor_record_offset":58544,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":522,"record_offset":59336,"parent_record_offset":59308,"ancestor_record_offset":58544,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":522,"record_offset":59308,"parent_record_offset":59280,"ancestor_record_offset":58544,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":522,"record_offset":61808,"parent_record_offset":61716,"ancestor_record_offset":58544,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":522,"record_offset":61832,"parent_record_offset":61808,"ancestor_record_offset":58544,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":522,"record_offset":61172,"parent_record_offset":59812,"ancestor_record_offset":58544,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":522,"record_offset":59772,"parent_record_offset":58816,"ancestor_record_offset":58544,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":522,"record_offset":61228,"parent_record_offset":59812,"ancestor_record_offset":58544,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":522,"record_offset":59568,"parent_record_offset":59544,"ancestor_record_offset":58544,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":522,"record_offset":61716,"parent_record_offset":61676,"ancestor_record_offset":58544,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":522,"record_offset":62124,"parent_record_offset":62100,"ancestor_record_offset":58544,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":522,"record_offset":61768,"parent_record_offset":61716,"ancestor_record_offset":58544,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":522,"record_offset":62100,"parent_record_offset":61920,"ancestor_record_offset":58544,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":522,"record_offset":60168,"parent_record_offset":60140,"ancestor_record_offset":58544,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":522,"record_offset":60024,"parent_record_offset":59904,"ancestor_record_offset":58544,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":522,"record_offset":61320,"parent_record_offset":61284,"ancestor_record_offset":58544,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":522,"record_offset":59940,"parent_record_offset":59904,"ancestor_record_offset":58544,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":522,"record_offset":60048,"parent_record_offset":60024,"ancestor_record_offset":58544,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":522,"record_offset":59964,"parent_record_offset":59940,"ancestor_record_offset":58544,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":522,"record_offset":61344,"parent_record_offset":61320,"ancestor_record_offset":58544,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":522,"record_offset":59676,"parent_record_offset":59652,"ancestor_record_offset":58544,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":522,"record_offset":62988,"parent_record_offset":62956,"ancestor_record_offset":58544,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":522,"record_offset":59628,"parent_record_offset":59604,"ancestor_record_offset":58544,"inlinee":4660,"source_line":88,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":522,"record_offset":59244,"parent_record_offset":59216,"ancestor_record_offset":58544,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":522,"record_offset":59280,"parent_record_offset":59244,"ancestor_record_offset":58544,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":522,"record_offset":59216,"parent_record_offset":58996,"ancestor_record_offset":58544,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":522,"record_offset":59076,"parent_record_offset":58996,"ancestor_record_offset":58544,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":522,"record_offset":59380,"parent_record_offset":58996,"ancestor_record_offset":58544,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":522,"record_offset":59160,"parent_record_offset":58996,"ancestor_record_offset":58544,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":522,"record_offset":58896,"parent_record_offset":58860,"ancestor_record_offset":58544,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":522,"record_offset":58920,"parent_record_offset":58896,"ancestor_record_offset":58544,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":522,"record_offset":59604,"parent_record_offset":59496,"ancestor_record_offset":58544,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":522,"record_offset":59544,"parent_record_offset":59520,"ancestor_record_offset":58544,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":522,"record_offset":59464,"parent_record_offset":58996,"ancestor_record_offset":58544,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":522,"record_offset":59520,"parent_record_offset":59496,"ancestor_record_offset":58544,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":522,"record_offset":58972,"parent_record_offset":58948,"ancestor_record_offset":58544,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":522,"record_offset":58948,"parent_record_offset":58920,"ancestor_record_offset":58544,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":522,"record_offset":58996,"parent_record_offset":58972,"ancestor_record_offset":58544,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":522,"record_offset":59048,"parent_record_offset":58996,"ancestor_record_offset":58544,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":522,"record_offset":59496,"parent_record_offset":58996,"ancestor_record_offset":58544,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":522,"record_offset":62772,"parent_record_offset":62732,"ancestor_record_offset":58544,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":522,"record_offset":62828,"parent_record_offset":62732,"ancestor_record_offset":58544,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":522,"record_offset":60112,"parent_record_offset":60084,"ancestor_record_offset":58544,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":522,"record_offset":60140,"parent_record_offset":60112,"ancestor_record_offset":58544,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":522,"record_offset":59868,"parent_record_offset":59812,"ancestor_record_offset":58544,"inlinee":8101,"source_line":367,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":522,"record_offset":60084,"parent_record_offset":59868,"ancestor_record_offset":58544,"inlinee":7022,"source_line":682,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$31::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":522,"record_offset":61256,"parent_record_offset":61228,"ancestor_record_offset":58544,"inlinee":7028,"source_line":816,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":522,"record_offset":59904,"parent_record_offset":59868,"ancestor_record_offset":58544,"inlinee":7017,"source_line":94,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":522,"record_offset":61284,"parent_record_offset":61256,"ancestor_record_offset":58544,"inlinee":7029,"source_line":110,"archive":"_pdb/B/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":522,"record_offset":59996,"parent_record_offset":59904,"ancestor_record_offset":58544,"inlinee":7019,"source_line":47,"archive":"_pdb/B/modules/00522.jsonl.gz"}
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
// 完整结构及原始类型记录: shader_refactoring.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shader_refactoring.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/shader_refactoring.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: shader_refactoring.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::shader_refactoring::render_shader_refactoring
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":522,"record_offset":62860,"parent_record_offset":58544,"ancestor_record_offset":58544,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00522.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":522,"record_offset":62908,"parent_record_offset":58544,"ancestor_record_offset":58544,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00522.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"e968d088fd6cd245d836e1af","relative_path":"_shader_refactoring.cs.hlsl_capture_0.bin","sha256":"7ab02ab729147c1f38f90fd82ecb6a5666d2bc56a6ba2ea89f631709c308e7de","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"shader_refactoring","prefix_candidates":["shader_refactoring"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"5156a52ca50c212a7a4ddcfd","relative_path":"_shader_refactoring.cs.hlsl_compare_0.bin","sha256":"5944cae4559e57ca4514032f4ae396a24d43652a2a36e1bcb5989ff946e97823","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"shader_refactoring","prefix_candidates":["shader_refactoring"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _shader_refactoring.cs.hlsl_compare_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"compare","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"rendered_tex","storage_class":"UniformConstant","type":{"id":28,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,10]},"decorations":[],"pointee":{"id":10,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[15,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"reference_tex","storage_class":"UniformConstant","type":{"id":29,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,12]},"decorations":[],"pointee":{"id":12,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[15,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"feedback_tex","storage_class":"UniformConstant","type":{"id":29,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,12]},"decorations":[],"pointee":{"id":12,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[15,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":31,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,30]},"decorations":[],"pointee":{"id":30,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[16,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _shader_refactoring.cs.hlsl_capture_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":1,"entry_name":"capture","interface_ids":[2]}],"execution_modes":[{"entry_id":1,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"rendered_tex","storage_class":"UniformConstant","type":{"id":17,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,9]},"decorations":[],"pointee":{"id":9,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[13,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"reference_tex","storage_class":"UniformConstant","type":{"id":18,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,11]},"decorations":[],"pointee":{"id":11,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[13,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":20,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,19]},"decorations":[],"pointee":{"id":19,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[14,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5165081,"position_among_mapped_render_targets":59,"mapped_render_target_count":59,"previous_callsite":5165067,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shader_refactoring.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::timelapse_replay::InterruptTimelapse>,bevy_ecs::event::writer::EventWriter<country_core::systems::timelapse_replay::TimelapseReplayFinished>,bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::timelapse_replay::InterruptTimelapse>", "bevy_ecs::event::writer::EventWriter<country_core::systems::timelapse_replay::TimelapseReplayFinished>", "bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::timelapse_replay::InterruptTimelapse>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::timelapse_replay::TimelapseReplayFinished>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::event::reader::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >", "bevy_ecs::event::reader::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::shader_refactoring::render_shader_refactoring
// 信息 ID: 69c29789a7116aac4a694003
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ed019 -> 0x98f9f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
