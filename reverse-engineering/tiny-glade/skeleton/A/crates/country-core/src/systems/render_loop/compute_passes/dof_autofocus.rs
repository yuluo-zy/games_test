// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/dof_autofocus.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 61d66b63346b7c5e5098ce06278f74fd6028a864486a5c38d706a61056aeab06
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: dof_autofocus.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 389
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof_autofocus::calculate_dof_autofocus
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":389,"record_offset":21132,"rva":5305328,"va":5374014448}
// 编译实例 RVAs: 0x50f3f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof_autofocus.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1257903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374014448,"raw_hex":"e8af311300","size":5,"soft_fail":false,"target_va":5375274368,"va":5374016460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof_autofocus.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1298095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374014448,"raw_hex":"e8afce1300","size":5,"soft_fail":false,"target_va":5375314864,"va":5374016764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dof_autofocus.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1961582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374014448,"raw_hex":"e86eee1d00","size":5,"soft_fail":false,"target_va":5375978624,"va":5374017037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dof_autofocus.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2223130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374014448,"raw_hex":"e81aec2100","size":5,"soft_fail":false,"target_va":5376239920,"va":5374016785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof_autofocus.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9168458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374014448,"raw_hex":"e84ae68b00","size":5,"soft_fail":false,"target_va":5383184544,"va":5374016081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd3630
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof_autofocus.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9190061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374014448,"raw_hex":"e8ad3a8c00","size":5,"soft_fail":false,"target_va":5383206448,"va":5374016382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd4110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof_autofocus.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9192824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374014448,"raw_hex":"e878458c00","size":5,"soft_fail":false,"target_va":5383209232,"va":5374016403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof_autofocus.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9212095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374014448,"raw_hex":"e8bf908c00","size":5,"soft_fail":false,"target_va":5383228352,"va":5374016252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8ee0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dof_autofocus.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9212926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374014448,"raw_hex":"e8fe938c00","size":5,"soft_fail":false,"target_va":5383229152,"va":5374016221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd9070
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: dof_autofocus.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9213354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374014448,"raw_hex":"e8aa958c00","size":5,"soft_fail":false,"target_va":5383229552,"va":5374016193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddb570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof_autofocus.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9222882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374014448,"raw_hex":"e8e2ba8c00","size":5,"soft_fail":false,"target_va":5383239024,"va":5374016137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dof_autofocus.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29577193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374014448,"raw_hex":"e8e94fc301","size":5,"soft_fail":false,"target_va":5403593360,"va":5374016162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t28962044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374014448,"raw_hex":"e8fcecb901","size":5,"soft_fail":false,"target_va":5402978880,"va":5374016831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":389,"record_offset":29688,"parent_record_offset":21132,"ancestor_record_offset":21132,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":389,"record_offset":29640,"parent_record_offset":21132,"ancestor_record_offset":21132,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":389,"record_offset":29564,"parent_record_offset":21132,"ancestor_record_offset":21132,"inlinee":10166,"source_line":666,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::WindowSize::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":389,"record_offset":29480,"parent_record_offset":21132,"ancestor_record_offset":21132,"inlinee":21913,"source_line":35,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::WindowSize::aspect_ratio
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":389,"record_offset":27664,"parent_record_offset":21132,"ancestor_record_offset":21132,"inlinee":21907,"source_line":43,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::MainCamera::focus
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":389,"record_offset":26048,"parent_record_offset":21132,"ancestor_record_offset":21132,"inlinee":22038,"source_line":433,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":389,"record_offset":21344,"parent_record_offset":21132,"ancestor_record_offset":21132,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":389,"record_offset":27808,"parent_record_offset":21132,"ancestor_record_offset":21132,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":389,"record_offset":27696,"parent_record_offset":21132,"ancestor_record_offset":21132,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":389,"record_offset":27752,"parent_record_offset":21132,"ancestor_record_offset":21132,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":389,"record_offset":21400,"parent_record_offset":21372,"ancestor_record_offset":21132,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":389,"record_offset":22832,"parent_record_offset":22800,"ancestor_record_offset":21132,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":389,"record_offset":22776,"parent_record_offset":22388,"ancestor_record_offset":21132,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":389,"record_offset":22800,"parent_record_offset":22776,"ancestor_record_offset":21132,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":389,"record_offset":22416,"parent_record_offset":22388,"ancestor_record_offset":21132,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":389,"record_offset":22464,"parent_record_offset":22440,"ancestor_record_offset":21132,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":389,"record_offset":22440,"parent_record_offset":22416,"ancestor_record_offset":21132,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":389,"record_offset":22556,"parent_record_offset":22492,"ancestor_record_offset":21132,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":389,"record_offset":22644,"parent_record_offset":22620,"ancestor_record_offset":21132,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":389,"record_offset":21372,"parent_record_offset":21344,"ancestor_record_offset":21132,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":389,"record_offset":22388,"parent_record_offset":21372,"ancestor_record_offset":21132,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":389,"record_offset":22120,"parent_record_offset":22096,"ancestor_record_offset":21132,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":389,"record_offset":22168,"parent_record_offset":22144,"ancestor_record_offset":21132,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":389,"record_offset":22096,"parent_record_offset":22072,"ancestor_record_offset":21132,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":389,"record_offset":21588,"parent_record_offset":21564,"ancestor_record_offset":21132,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":389,"record_offset":21672,"parent_record_offset":21648,"ancestor_record_offset":21132,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":389,"record_offset":21612,"parent_record_offset":21588,"ancestor_record_offset":21132,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":389,"record_offset":21784,"parent_record_offset":21760,"ancestor_record_offset":21132,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":389,"record_offset":27776,"parent_record_offset":27752,"ancestor_record_offset":21132,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":389,"record_offset":27832,"parent_record_offset":27808,"ancestor_record_offset":21132,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":389,"record_offset":27720,"parent_record_offset":27696,"ancestor_record_offset":21132,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":389,"record_offset":29608,"parent_record_offset":29564,"ancestor_record_offset":21132,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":389,"record_offset":22580,"parent_record_offset":22556,"ancestor_record_offset":21132,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":389,"record_offset":22356,"parent_record_offset":21400,"ancestor_record_offset":21132,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":389,"record_offset":22620,"parent_record_offset":22440,"ancestor_record_offset":21132,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":389,"record_offset":22876,"parent_record_offset":21344,"ancestor_record_offset":21132,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":389,"record_offset":22724,"parent_record_offset":22700,"ancestor_record_offset":21132,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":389,"record_offset":29792,"parent_record_offset":29720,"ancestor_record_offset":21132,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":389,"record_offset":22008,"parent_record_offset":21984,"ancestor_record_offset":21132,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":389,"record_offset":21928,"parent_record_offset":21904,"ancestor_record_offset":21132,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":389,"record_offset":22492,"parent_record_offset":22464,"ancestor_record_offset":21132,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":389,"record_offset":22528,"parent_record_offset":22492,"ancestor_record_offset":21132,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":389,"record_offset":22700,"parent_record_offset":22668,"ancestor_record_offset":21132,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":389,"record_offset":22144,"parent_record_offset":22120,"ancestor_record_offset":21132,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":389,"record_offset":29720,"parent_record_offset":29688,"ancestor_record_offset":21132,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::CameraModeStates::current_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":389,"record_offset":26072,"parent_record_offset":26048,"ancestor_record_offset":21132,"inlinee":22039,"source_line":149,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":389,"record_offset":22072,"parent_record_offset":22048,"ancestor_record_offset":21132,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":389,"record_offset":21424,"parent_record_offset":21400,"ancestor_record_offset":21132,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":389,"record_offset":21448,"parent_record_offset":21424,"ancestor_record_offset":21132,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":389,"record_offset":22048,"parent_record_offset":21852,"ancestor_record_offset":21132,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":389,"record_offset":21960,"parent_record_offset":21876,"ancestor_record_offset":21132,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":389,"record_offset":21904,"parent_record_offset":21876,"ancestor_record_offset":21132,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":389,"record_offset":22304,"parent_record_offset":21524,"ancestor_record_offset":21132,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":389,"record_offset":21876,"parent_record_offset":21852,"ancestor_record_offset":21132,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":389,"record_offset":21500,"parent_record_offset":21476,"ancestor_record_offset":21132,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":389,"record_offset":21476,"parent_record_offset":21448,"ancestor_record_offset":21132,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":389,"record_offset":21524,"parent_record_offset":21500,"ancestor_record_offset":21132,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":389,"record_offset":21732,"parent_record_offset":21704,"ancestor_record_offset":21132,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":389,"record_offset":21760,"parent_record_offset":21732,"ancestor_record_offset":21132,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":389,"record_offset":21820,"parent_record_offset":21704,"ancestor_record_offset":21132,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":389,"record_offset":21704,"parent_record_offset":21524,"ancestor_record_offset":21132,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":389,"record_offset":21852,"parent_record_offset":21524,"ancestor_record_offset":21132,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":389,"record_offset":21564,"parent_record_offset":21524,"ancestor_record_offset":21132,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":389,"record_offset":21648,"parent_record_offset":21524,"ancestor_record_offset":21132,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":389,"record_offset":22220,"parent_record_offset":21524,"ancestor_record_offset":21132,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":389,"record_offset":22668,"parent_record_offset":22644,"ancestor_record_offset":21132,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00389.jsonl.gz"}
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
// 完整结构及原始类型记录: dof_autofocus.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::MappedStorageBuffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo>", "constructor": "country_core::render::ssbo::MappedStorageBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::MappedStorageBuffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::MappedStorageBuffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$1::as_any<country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$1::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$1::as_any_mut<country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$1::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$4::new_storage_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo>", "constructor": "country_core::render::ssbo::impl$4::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: dof_autofocus.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/dof_autofocus.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: dof_autofocus.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::dof_autofocus::calculate_dof_autofocus
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":389,"record_offset":29564,"parent_record_offset":21132,"ancestor_record_offset":21132,"inlinee":10166,"source_line":666,"archive":"_pdb/A/modules/00389.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":389,"record_offset":29640,"parent_record_offset":21132,"ancestor_record_offset":21132,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00389.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"f14e9682a93c2937231ee6fa","relative_path":"_dof_autofocus.cs.hlsl_cs_main_0.bin","sha256":"8132f6aa57601509b830a927ebf7427c7c8401dc88a51ad5b645b34ca9d102f8","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof_autofocus","prefix_candidates":["dof_autofocus","dof_autofocus"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_autofocus.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"frame_constants","storage_class":"Uniform","type":{"id":63,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,5]},"decorations":[],"pointee":{"id":5,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[53,53,53,53,53,53,53,53,27,26,26,27,60,27,26,32,26,32,27,32,27,53,20]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":53,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":53,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":53,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":27,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":26,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":26,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":27,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":60,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":27,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":26,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":32,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":26,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":32,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":27,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":32,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":27,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":53,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":20,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"$Globals","storage_class":"Uniform","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,9]},"decorations":[],"pointee":{"id":9,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[60,31,27,27,27,26,27,26]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"depth_tex_dims","type_id":60,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"point_uv","type_id":31,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"use_dual_focus","type_id":27,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"focus_lock_request","type_id":27,"decorations":[{"decoration":"Offset","operands":[20]}]},{"index":4,"name":"focus_is_locked","type_id":27,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":5,"name":"focus_box_height","type_id":26,"decorations":[{"decoration":"Offset","operands":[28]}]},{"index":6,"name":"use_tilt_shift","type_id":27,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":7,"name":"aperture_size","type_id":26,"decorations":[{"decoration":"Offset","operands":[36]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,8]},"decorations":[],"pointee":{"id":8,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[26,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"autofocus_buf","storage_class":"Uniform","type":{"id":66,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,12]},"decorations":[],"pointee":{"id":12,"name":"type.RWStructuredBuffer.DofFocus","record":{"opcode":"OpTypeStruct","operands":[21]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":21,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"focus_readback_buf","storage_class":"Uniform","type":{"id":67,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,15]},"decorations":[],"pointee":{"id":15,"name":"type.RWStructuredBuffer.DofFocusReadback","record":{"opcode":"OpTypeStruct","operands":[22]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":22,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,71]},"decorations":[],"pointee":{"id":71,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[27,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":73,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,27]},"decorations":[],"pointee":{"id":27,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3139220,"position_among_mapped_render_targets":35,"mapped_render_target_count":70,"previous_callsite":3139191,"next_callsite":3139249,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/visibility_queries.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/mouse_depth_value.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: dof_autofocus.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof_autofocus::calculate_dof_autofocus
// 信息 ID: 58c310e41c2cda4967c19827
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe694 -> 0x50f3f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
