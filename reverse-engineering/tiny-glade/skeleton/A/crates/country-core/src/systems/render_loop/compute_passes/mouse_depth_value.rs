// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/mouse_depth_value.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5ec61aaf44e41892650a884b4b06a0a4f1fec2f5693db1b4848955f76d974463
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mouse_depth_value.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 387
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":387,"record_offset":36528,"rva":5279424,"va":5373988544}
// 编译实例 RVAs: 0x508ec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mouse_depth_value.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1285353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373988544,"raw_hex":"e8e99c1300","size":5,"soft_fail":false,"target_va":5375274368,"va":5373989010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mouse_depth_value.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1325138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373988544,"raw_hex":"e852381400","size":5,"soft_fail":false,"target_va":5375314864,"va":5373989721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mouse_depth_value.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1988747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373988544,"raw_hex":"e88b581e00","size":5,"soft_fail":false,"target_va":5375978624,"va":5373989872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mouse_depth_value.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2250173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373988544,"raw_hex":"e8bd552200","size":5,"soft_fail":false,"target_va":5376239920,"va":5373989742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mouse_depth_value.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9195604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373988544,"raw_hex":"e854508c00","size":5,"soft_fail":false,"target_va":5383184544,"va":5373988935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd5ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mouse_depth_value.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9227915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373988544,"raw_hex":"e88bce8c00","size":5,"soft_fail":false,"target_va":5383216880,"va":5373988960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddb570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mouse_depth_value.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9249934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373988544,"raw_hex":"e88e248d00","size":5,"soft_fail":false,"target_va":5383239024,"va":5373989085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: mouse_depth_value.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29604323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373988544,"raw_hex":"e8e3b9c301","size":5,"soft_fail":false,"target_va":5403593360,"va":5373989032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28989089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373988544,"raw_hex":"e8a156ba01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373989786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":387,"record_offset":40080,"parent_record_offset":36528,"ancestor_record_offset":36528,"inlinee":21933,"source_line":452,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":387,"record_offset":38284,"parent_record_offset":36528,"ancestor_record_offset":36528,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":387,"record_offset":38332,"parent_record_offset":36528,"ancestor_record_offset":36528,"inlinee":10166,"source_line":666,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::WindowSize::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":387,"record_offset":40052,"parent_record_offset":36528,"ancestor_record_offset":36528,"inlinee":21913,"source_line":35,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":387,"record_offset":36720,"parent_record_offset":36528,"ancestor_record_offset":36528,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":387,"record_offset":36776,"parent_record_offset":36748,"ancestor_record_offset":36528,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":387,"record_offset":38208,"parent_record_offset":38176,"ancestor_record_offset":36528,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":387,"record_offset":38152,"parent_record_offset":37764,"ancestor_record_offset":36528,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":387,"record_offset":38176,"parent_record_offset":38152,"ancestor_record_offset":36528,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":387,"record_offset":37792,"parent_record_offset":37764,"ancestor_record_offset":36528,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":387,"record_offset":37840,"parent_record_offset":37816,"ancestor_record_offset":36528,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":387,"record_offset":37816,"parent_record_offset":37792,"ancestor_record_offset":36528,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":387,"record_offset":37932,"parent_record_offset":37868,"ancestor_record_offset":36528,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":387,"record_offset":38020,"parent_record_offset":37996,"ancestor_record_offset":36528,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":387,"record_offset":36748,"parent_record_offset":36720,"ancestor_record_offset":36528,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":387,"record_offset":37764,"parent_record_offset":36748,"ancestor_record_offset":36528,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":387,"record_offset":37496,"parent_record_offset":37472,"ancestor_record_offset":36528,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":387,"record_offset":37544,"parent_record_offset":37520,"ancestor_record_offset":36528,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":387,"record_offset":37472,"parent_record_offset":37448,"ancestor_record_offset":36528,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":387,"record_offset":36964,"parent_record_offset":36940,"ancestor_record_offset":36528,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":387,"record_offset":37048,"parent_record_offset":37024,"ancestor_record_offset":36528,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":387,"record_offset":36988,"parent_record_offset":36964,"ancestor_record_offset":36528,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":387,"record_offset":37160,"parent_record_offset":37136,"ancestor_record_offset":36528,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":387,"record_offset":38376,"parent_record_offset":38332,"ancestor_record_offset":36528,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":387,"record_offset":37956,"parent_record_offset":37932,"ancestor_record_offset":36528,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":387,"record_offset":37732,"parent_record_offset":36776,"ancestor_record_offset":36528,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":387,"record_offset":37996,"parent_record_offset":37816,"ancestor_record_offset":36528,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":387,"record_offset":38252,"parent_record_offset":36720,"ancestor_record_offset":36528,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":387,"record_offset":38100,"parent_record_offset":38076,"ancestor_record_offset":36528,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":40184,"parent_record_offset":40112,"ancestor_record_offset":36528,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":387,"record_offset":37384,"parent_record_offset":37360,"ancestor_record_offset":36528,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":387,"record_offset":37304,"parent_record_offset":37280,"ancestor_record_offset":36528,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":387,"record_offset":37868,"parent_record_offset":37840,"ancestor_record_offset":36528,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":387,"record_offset":37904,"parent_record_offset":37868,"ancestor_record_offset":36528,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":387,"record_offset":38076,"parent_record_offset":38044,"ancestor_record_offset":36528,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":387,"record_offset":37520,"parent_record_offset":37496,"ancestor_record_offset":36528,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":387,"record_offset":40112,"parent_record_offset":40080,"ancestor_record_offset":36528,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":387,"record_offset":37448,"parent_record_offset":37424,"ancestor_record_offset":36528,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":387,"record_offset":36800,"parent_record_offset":36776,"ancestor_record_offset":36528,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":387,"record_offset":36824,"parent_record_offset":36800,"ancestor_record_offset":36528,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":387,"record_offset":37424,"parent_record_offset":37228,"ancestor_record_offset":36528,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":387,"record_offset":37336,"parent_record_offset":37252,"ancestor_record_offset":36528,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":387,"record_offset":37280,"parent_record_offset":37252,"ancestor_record_offset":36528,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":387,"record_offset":37680,"parent_record_offset":36900,"ancestor_record_offset":36528,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":387,"record_offset":37252,"parent_record_offset":37228,"ancestor_record_offset":36528,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":387,"record_offset":36876,"parent_record_offset":36852,"ancestor_record_offset":36528,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":387,"record_offset":36852,"parent_record_offset":36824,"ancestor_record_offset":36528,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":387,"record_offset":36900,"parent_record_offset":36876,"ancestor_record_offset":36528,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":387,"record_offset":37108,"parent_record_offset":37080,"ancestor_record_offset":36528,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":387,"record_offset":37136,"parent_record_offset":37108,"ancestor_record_offset":36528,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":387,"record_offset":37196,"parent_record_offset":37080,"ancestor_record_offset":36528,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":387,"record_offset":37080,"parent_record_offset":36900,"ancestor_record_offset":36528,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":387,"record_offset":37228,"parent_record_offset":36900,"ancestor_record_offset":36528,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":387,"record_offset":36940,"parent_record_offset":36900,"ancestor_record_offset":36528,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":387,"record_offset":37024,"parent_record_offset":36900,"ancestor_record_offset":36528,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":387,"record_offset":37596,"parent_record_offset":36900,"ancestor_record_offset":36528,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":387,"record_offset":38044,"parent_record_offset":38020,"ancestor_record_offset":36528,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00387.jsonl.gz"}
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
// 完整结构及原始类型记录: mouse_depth_value.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mouse_depth_value.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/mouse_depth_value.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: mouse_depth_value.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":387,"record_offset":38284,"parent_record_offset":36528,"ancestor_record_offset":36528,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00387.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":387,"record_offset":38332,"parent_record_offset":36528,"ancestor_record_offset":36528,"inlinee":10166,"source_line":666,"archive":"_pdb/A/modules/00387.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3139249,"position_among_mapped_render_targets":36,"mapped_render_target_count":70,"previous_callsite":3139220,"next_callsite":3139297,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/dof_autofocus.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mouse_depth_value.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute
// 信息 ID: 124bfa1a1fe1a9b0bc7c72a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe6b1 -> 0x508ec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
