// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/apply_volumetrics.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ffa960f2d26cc3b17d4289974e7f8ecdf520ac546ef084b92ce9cda29de225c1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: apply_volumetrics.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 473
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
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":473,"record_offset":9812,"rva":6694304,"va":5375403424}
// 编译实例 RVAs: 0x6625a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1189506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375403424,"raw_hex":"e87ed9edff","size":5,"soft_fail":false,"target_va":5374214416,"va":5375403917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-129692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375403424,"raw_hex":"e86405feff","size":5,"soft_fail":false,"target_va":5375274368,"va":5375404055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-128181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375403424,"raw_hex":"e84b0bfeff","size":5,"soft_fail":false,"target_va":5375275808,"va":5375403984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-89596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375403424,"raw_hex":"e804a2feff","size":5,"soft_fail":false,"target_va":5375314864,"va":5375404455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t573950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375403424,"raw_hex":"e8fec10800","size":5,"soft_fail":false,"target_va":5375978624,"va":5375404669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t835439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375403424,"raw_hex":"e86fbf0c00","size":5,"soft_fail":false,"target_va":5376239920,"va":5375404476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dcab40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7767015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375403424,"raw_hex":"e8e7837600","size":5,"soft_fail":false,"target_va":5383170880,"va":5375403860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7780697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375403424,"raw_hex":"e859b97600","size":5,"soft_fail":false,"target_va":5383184544,"va":5375403842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7824236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375403424,"raw_hex":"e86c637700","size":5,"soft_fail":false,"target_va":5383228352,"va":5375404111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8ee0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7825012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375403424,"raw_hex":"e874667700","size":5,"soft_fail":false,"target_va":5383229152,"va":5375404135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28189421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375403424,"raw_hex":"e8ed22ae01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375403934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27574353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375403424,"raw_hex":"e851c0a401","size":5,"soft_fail":false,"target_va":5402978880,"va":5375404522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t938015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375403424,"raw_hex":"e81f500e00","size":5,"soft_fail":false,"target_va":5376341760,"va":5375403740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":473,"record_offset":11768,"parent_record_offset":9812,"ancestor_record_offset":9812,"inlinee":21933,"source_line":452,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":473,"record_offset":11672,"parent_record_offset":9812,"ancestor_record_offset":9812,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":473,"record_offset":11624,"parent_record_offset":9812,"ancestor_record_offset":9812,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::MainCamera::mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":473,"record_offset":11596,"parent_record_offset":9812,"ancestor_record_offset":9812,"inlinee":21922,"source_line":429,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":473,"record_offset":10004,"parent_record_offset":9812,"ancestor_record_offset":9812,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":473,"record_offset":11568,"parent_record_offset":9812,"ancestor_record_offset":9812,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":473,"record_offset":10060,"parent_record_offset":10032,"ancestor_record_offset":9812,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":473,"record_offset":11492,"parent_record_offset":11460,"ancestor_record_offset":9812,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":473,"record_offset":11436,"parent_record_offset":11048,"ancestor_record_offset":9812,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":473,"record_offset":11460,"parent_record_offset":11436,"ancestor_record_offset":9812,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":473,"record_offset":11076,"parent_record_offset":11048,"ancestor_record_offset":9812,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":473,"record_offset":11124,"parent_record_offset":11100,"ancestor_record_offset":9812,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":473,"record_offset":11100,"parent_record_offset":11076,"ancestor_record_offset":9812,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":473,"record_offset":11216,"parent_record_offset":11152,"ancestor_record_offset":9812,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":473,"record_offset":11304,"parent_record_offset":11280,"ancestor_record_offset":9812,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":473,"record_offset":10032,"parent_record_offset":10004,"ancestor_record_offset":9812,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":473,"record_offset":11048,"parent_record_offset":10032,"ancestor_record_offset":9812,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":473,"record_offset":10780,"parent_record_offset":10756,"ancestor_record_offset":9812,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":473,"record_offset":10828,"parent_record_offset":10804,"ancestor_record_offset":9812,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":473,"record_offset":10756,"parent_record_offset":10732,"ancestor_record_offset":9812,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":473,"record_offset":10248,"parent_record_offset":10224,"ancestor_record_offset":9812,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":473,"record_offset":10332,"parent_record_offset":10308,"ancestor_record_offset":9812,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":473,"record_offset":10272,"parent_record_offset":10248,"ancestor_record_offset":9812,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":473,"record_offset":10444,"parent_record_offset":10420,"ancestor_record_offset":9812,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":11240,"parent_record_offset":11216,"ancestor_record_offset":9812,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":11016,"parent_record_offset":10060,"ancestor_record_offset":9812,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":11280,"parent_record_offset":11100,"ancestor_record_offset":9812,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":11536,"parent_record_offset":10004,"ancestor_record_offset":9812,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":473,"record_offset":11384,"parent_record_offset":11360,"ancestor_record_offset":9812,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":11868,"parent_record_offset":11796,"ancestor_record_offset":9812,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":473,"record_offset":10668,"parent_record_offset":10644,"ancestor_record_offset":9812,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":473,"record_offset":10588,"parent_record_offset":10564,"ancestor_record_offset":9812,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":473,"record_offset":11152,"parent_record_offset":11124,"ancestor_record_offset":9812,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":473,"record_offset":11188,"parent_record_offset":11152,"ancestor_record_offset":9812,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":473,"record_offset":11360,"parent_record_offset":11328,"ancestor_record_offset":9812,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":473,"record_offset":10804,"parent_record_offset":10780,"ancestor_record_offset":9812,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":473,"record_offset":11796,"parent_record_offset":11768,"ancestor_record_offset":9812,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":473,"record_offset":10732,"parent_record_offset":10708,"ancestor_record_offset":9812,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":473,"record_offset":10084,"parent_record_offset":10060,"ancestor_record_offset":9812,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":473,"record_offset":10108,"parent_record_offset":10084,"ancestor_record_offset":9812,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":473,"record_offset":10708,"parent_record_offset":10512,"ancestor_record_offset":9812,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":473,"record_offset":10620,"parent_record_offset":10536,"ancestor_record_offset":9812,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":473,"record_offset":10564,"parent_record_offset":10536,"ancestor_record_offset":9812,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":473,"record_offset":10964,"parent_record_offset":10184,"ancestor_record_offset":9812,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":473,"record_offset":10536,"parent_record_offset":10512,"ancestor_record_offset":9812,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":473,"record_offset":10160,"parent_record_offset":10136,"ancestor_record_offset":9812,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":473,"record_offset":10136,"parent_record_offset":10108,"ancestor_record_offset":9812,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":473,"record_offset":10184,"parent_record_offset":10160,"ancestor_record_offset":9812,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":473,"record_offset":10392,"parent_record_offset":10364,"ancestor_record_offset":9812,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":473,"record_offset":10420,"parent_record_offset":10392,"ancestor_record_offset":9812,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":473,"record_offset":10480,"parent_record_offset":10364,"ancestor_record_offset":9812,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":473,"record_offset":10364,"parent_record_offset":10184,"ancestor_record_offset":9812,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":473,"record_offset":10512,"parent_record_offset":10184,"ancestor_record_offset":9812,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":473,"record_offset":10224,"parent_record_offset":10184,"ancestor_record_offset":9812,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":473,"record_offset":10308,"parent_record_offset":10184,"ancestor_record_offset":9812,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":473,"record_offset":10880,"parent_record_offset":10184,"ancestor_record_offset":9812,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":473,"record_offset":11328,"parent_record_offset":11304,"ancestor_record_offset":9812,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00473.jsonl.gz"}
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
// 完整结构及原始类型记录: apply_volumetrics.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: apply_volumetrics.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/apply_volumetrics.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: apply_volumetrics.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":473,"record_offset":11624,"parent_record_offset":9812,"ancestor_record_offset":9812,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00473.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":473,"record_offset":11672,"parent_record_offset":9812,"ancestor_record_offset":9812,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00473.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"e65e445f5c4de7911a3a5a9e","relative_path":"_apply_volumetrics.cs.hlsl_cs_main_0.bin","sha256":"e73973302f4e9d59e32d4f4197d49585ca1fe17fe100dd4898d4d05e78728305","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"apply_volumetrics","prefix_candidates":["apply_volumetrics","volumetric_","apply_volumetrics"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _apply_volumetrics.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"sampler_shadow_linear","storage_class":"UniformConstant","type":{"id":90,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,4]},"decorations":[],"pointee":{"id":4,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":8,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[8]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"frame_constants","storage_class":"Uniform","type":{"id":93,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,6]},"decorations":[],"pointee":{"id":6,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[73,73,73,73,73,73,73,73,25,23,23,25,49,25,23,31,23,31,25,31,25,73,19]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":73,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":73,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":73,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":73,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":73,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":73,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":73,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":73,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":25,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":23,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":23,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":25,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":49,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":25,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":23,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":31,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":23,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":31,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":25,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":31,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":25,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":73,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":19,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"$Globals","storage_class":"Uniform","type":{"id":94,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[25,49,23,23,49,49]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":25,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"depth_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"global_density_scale","type_id":23,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"global_size_scale","type_id":23,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":4,"name":"blue_noise_texture_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[20]}]},{"index":5,"name":"color_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[32]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"shadow_map","storage_class":"UniformConstant","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,12]},"decorations":[],"pointee":{"id":12,"name":"type.2d.image.array","record":{"opcode":"OpTypeImage","operands":[23,1,2,1,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":96,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,9]},"decorations":[],"pointee":{"id":9,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[23,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":96,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,9]},"decorations":[],"pointee":{"id":9,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[23,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"color_tex","storage_class":"UniformConstant","type":{"id":97,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,16]},"decorations":[],"pointee":{"id":16,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[23,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,98]},"decorations":[],"pointee":{"id":98,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3140752,"position_among_mapped_render_targets":48,"mapped_render_target_count":70,"previous_callsite":3140672,"next_callsite":3140794,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/apply_screen_shadows.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/dof.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: apply_volumetrics.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics
// 信息 ID: 8e6aea2bf32ad6e9a43cd0dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fec90 -> 0x6625a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
