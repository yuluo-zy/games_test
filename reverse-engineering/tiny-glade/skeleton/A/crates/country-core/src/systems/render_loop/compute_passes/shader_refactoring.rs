// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/shader_refactoring.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 8aa0e05c8231ac0dc1ddd4ac94cb46afb2cfe99627bb0ade487f01765628c714
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shader_refactoring.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 412
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
// 函数符号: country_core::systems::render_loop::compute_passes::shader_refactoring::render_shader_refactoring
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":412,"record_offset":13752,"rva":5737088,"va":5374446208}
// 编译实例 RVAs: 0x578a80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405abac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t208487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e8672e0300","size":5,"soft_fail":false,"target_va":5374655168,"va":5374446676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t827364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e8e49f0c00","size":5,"soft_fail":false,"target_va":5375274368,"va":5374446999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t828719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e82fa50c00","size":5,"soft_fail":false,"target_va":5375275808,"va":5374447084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t867348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e8143c0d00","size":5,"soft_fail":false,"target_va":5375314864,"va":5374447511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1530744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e8785b1700","size":5,"soft_fail":false,"target_va":5375978624,"va":5374447875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1792377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e879591b00","size":5,"soft_fail":false,"target_va":5376239920,"va":5374447538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8737594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e83a538500","size":5,"soft_fail":false,"target_va":5383184544,"va":5374446945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shader_refactoring.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29146328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e8d8bcbc01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374447027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2916315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e8db7f2c00","size":5,"soft_fail":false,"target_va":5377364336,"va":5374448016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28531184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e8f059b301","size":5,"soft_fail":false,"target_va":5402978880,"va":5374447691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28531326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e87e5ab301","size":5,"soft_fail":false,"target_va":5402979152,"va":5374447821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28532381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e89d5eb301","size":5,"soft_fail":false,"target_va":5402980128,"va":5374447742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1895186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e812eb1c00","size":5,"soft_fail":false,"target_va":5376341760,"va":5374446569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1893901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e80de61c00","size":5,"soft_fail":false,"target_va":5376340512,"va":5374446606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1894076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e8bce61c00","size":5,"soft_fail":false,"target_va":5376340720,"va":5374446639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1894126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374446208,"raw_hex":"e8eee61c00","size":5,"soft_fail":false,"target_va":5376340752,"va":5374446621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":412,"record_offset":16328,"parent_record_offset":13752,"ancestor_record_offset":13752,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":412,"record_offset":13964,"parent_record_offset":13752,"ancestor_record_offset":13752,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":16440,"parent_record_offset":13752,"ancestor_record_offset":13752,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":412,"record_offset":15696,"parent_record_offset":13752,"ancestor_record_offset":13752,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":412,"record_offset":15992,"parent_record_offset":13752,"ancestor_record_offset":13752,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":412,"record_offset":15896,"parent_record_offset":13752,"ancestor_record_offset":13752,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":412,"record_offset":15848,"parent_record_offset":13752,"ancestor_record_offset":13752,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":412,"record_offset":15668,"parent_record_offset":13752,"ancestor_record_offset":13752,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":412,"record_offset":16352,"parent_record_offset":16328,"ancestor_record_offset":13752,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":412,"record_offset":13992,"parent_record_offset":13964,"ancestor_record_offset":13752,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":412,"record_offset":15504,"parent_record_offset":15480,"ancestor_record_offset":13752,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":412,"record_offset":15480,"parent_record_offset":15456,"ancestor_record_offset":13752,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":412,"record_offset":15428,"parent_record_offset":15396,"ancestor_record_offset":13752,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":412,"record_offset":15372,"parent_record_offset":14980,"ancestor_record_offset":13752,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":412,"record_offset":15396,"parent_record_offset":15372,"ancestor_record_offset":13752,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":412,"record_offset":15012,"parent_record_offset":14980,"ancestor_record_offset":13752,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":412,"record_offset":15456,"parent_record_offset":15396,"ancestor_record_offset":13752,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":15060,"parent_record_offset":15036,"ancestor_record_offset":13752,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":15036,"parent_record_offset":15012,"ancestor_record_offset":13752,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":15152,"parent_record_offset":15088,"ancestor_record_offset":13752,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":15240,"parent_record_offset":15216,"ancestor_record_offset":13752,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":412,"record_offset":15572,"parent_record_offset":15548,"ancestor_record_offset":13752,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":412,"record_offset":15548,"parent_record_offset":14980,"ancestor_record_offset":13752,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":412,"record_offset":14980,"parent_record_offset":13964,"ancestor_record_offset":13752,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":412,"record_offset":14712,"parent_record_offset":14688,"ancestor_record_offset":13752,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":412,"record_offset":14760,"parent_record_offset":14736,"ancestor_record_offset":13752,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":412,"record_offset":14688,"parent_record_offset":14664,"ancestor_record_offset":13752,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":412,"record_offset":14180,"parent_record_offset":14156,"ancestor_record_offset":13752,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":412,"record_offset":14264,"parent_record_offset":14240,"ancestor_record_offset":13752,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":412,"record_offset":14204,"parent_record_offset":14180,"ancestor_record_offset":13752,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":412,"record_offset":14376,"parent_record_offset":14352,"ancestor_record_offset":13752,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":15176,"parent_record_offset":15152,"ancestor_record_offset":13752,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":14948,"parent_record_offset":13992,"ancestor_record_offset":13752,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":15216,"parent_record_offset":15036,"ancestor_record_offset":13752,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":412,"record_offset":15320,"parent_record_offset":15296,"ancestor_record_offset":13752,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":412,"record_offset":15740,"parent_record_offset":15696,"ancestor_record_offset":13752,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":412,"record_offset":14600,"parent_record_offset":14576,"ancestor_record_offset":13752,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":412,"record_offset":14520,"parent_record_offset":14496,"ancestor_record_offset":13752,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":412,"record_offset":15088,"parent_record_offset":15060,"ancestor_record_offset":13752,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":412,"record_offset":15124,"parent_record_offset":15088,"ancestor_record_offset":13752,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":412,"record_offset":15296,"parent_record_offset":15264,"ancestor_record_offset":13752,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":412,"record_offset":15596,"parent_record_offset":15572,"ancestor_record_offset":13752,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":412,"record_offset":15620,"parent_record_offset":15596,"ancestor_record_offset":13752,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":412,"record_offset":14736,"parent_record_offset":14712,"ancestor_record_offset":13752,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":412,"record_offset":16024,"parent_record_offset":15992,"ancestor_record_offset":13752,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":412,"record_offset":14664,"parent_record_offset":14640,"ancestor_record_offset":13752,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":412,"record_offset":14016,"parent_record_offset":13992,"ancestor_record_offset":13752,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":412,"record_offset":14040,"parent_record_offset":14016,"ancestor_record_offset":13752,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":412,"record_offset":14640,"parent_record_offset":14444,"ancestor_record_offset":13752,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":14552,"parent_record_offset":14468,"ancestor_record_offset":13752,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":14496,"parent_record_offset":14468,"ancestor_record_offset":13752,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":14896,"parent_record_offset":14116,"ancestor_record_offset":13752,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":14468,"parent_record_offset":14444,"ancestor_record_offset":13752,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":14092,"parent_record_offset":14068,"ancestor_record_offset":13752,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":14068,"parent_record_offset":14040,"ancestor_record_offset":13752,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":14116,"parent_record_offset":14092,"ancestor_record_offset":13752,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":412,"record_offset":14324,"parent_record_offset":14296,"ancestor_record_offset":13752,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":412,"record_offset":14352,"parent_record_offset":14324,"ancestor_record_offset":13752,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":412,"record_offset":14412,"parent_record_offset":14296,"ancestor_record_offset":13752,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":412,"record_offset":14296,"parent_record_offset":14116,"ancestor_record_offset":13752,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":14444,"parent_record_offset":14116,"ancestor_record_offset":13752,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":412,"record_offset":14156,"parent_record_offset":14116,"ancestor_record_offset":13752,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":412,"record_offset":14240,"parent_record_offset":14116,"ancestor_record_offset":13752,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":412,"record_offset":14812,"parent_record_offset":14116,"ancestor_record_offset":13752,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":412,"record_offset":15264,"parent_record_offset":15240,"ancestor_record_offset":13752,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00412.jsonl.gz"}
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
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":412,"record_offset":15848,"parent_record_offset":13752,"ancestor_record_offset":13752,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00412.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":412,"record_offset":15896,"parent_record_offset":13752,"ancestor_record_offset":13752,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00412.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"d332a5a208d94c565b81df5a","relative_path":"_shader_refactoring.cs.hlsl_capture_0.bin","sha256":"51b53519f675e5bb89334c1f20b78364628720c0eb4b803d3bc019044bccde70","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"shader_refactoring","prefix_candidates":["shader_refactoring"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"71ac246714d1303ebb974141","relative_path":"_shader_refactoring.cs.hlsl_compare_0.bin","sha256":"b813939f3a85c81befb9cc6ae5dfadcdcdf955724043d39853236f123b6218a0","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"shader_refactoring","prefix_candidates":["shader_refactoring"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _shader_refactoring.cs.hlsl_compare_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"compare","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"rendered_tex","storage_class":"UniformConstant","type":{"id":22,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,4]},"decorations":[],"pointee":{"id":4,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"reference_tex","storage_class":"UniformConstant","type":{"id":23,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,6]},"decorations":[],"pointee":{"id":6,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"feedback_tex","storage_class":"UniformConstant","type":{"id":23,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,6]},"decorations":[],"pointee":{"id":6,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":25,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,24]},"decorations":[],"pointee":{"id":24,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[10,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _shader_refactoring.cs.hlsl_capture_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":1,"entry_name":"capture","interface_ids":[2]}],"execution_modes":[{"entry_id":1,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"rendered_tex","storage_class":"UniformConstant","type":{"id":11,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,3]},"decorations":[],"pointee":{"id":3,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[7,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"reference_tex","storage_class":"UniformConstant","type":{"id":12,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[7,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":14,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,13]},"decorations":[],"pointee":{"id":13,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[8,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3141718,"position_among_mapped_render_targets":54,"mapped_render_target_count":70,"previous_callsite":3141689,"next_callsite":3142766,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/final_post.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shader_refactoring.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::shader_refactoring::render_shader_refactoring
// 信息 ID: 169e9460c99ead27c0ac1892
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ff056 -> 0x578a80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
