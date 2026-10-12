// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/paths/mod.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 71a1da86bd9a5ef1195d8784b56ed50e62f6a7840bca6145d88b8ea0a3950bc8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 431
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_app; // namespace candidate
//   package identities: []
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
// 函数符号: country_core::systems::paths::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":431,"record_offset":112404,"rva":6062320,"va":5374771440}
// 编译实例 RVAs: 0x5c80f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400379d0
//     owner=unresolved_namespace; count=2
//     target_aliases=84 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5835727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374771440,"raw_hex":"e831f4a6ff","size":5,"soft_fail":false,"target_va":5368936912,"va":5374772634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140037d90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5833660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374771440,"raw_hex":"e844fca6ff","size":5,"soft_fail":false,"target_va":5368937872,"va":5374771527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140673a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t702690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374771440,"raw_hex":"e8e2b80a00","size":5,"soft_fail":false,"target_va":5375474320,"va":5374771625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140da8ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8260003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374771440,"raw_hex":"e8a3097e00","size":5,"soft_fail":false,"target_va":5383031520,"va":5374771512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141031320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10916026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374771440,"raw_hex":"e8ba90a600","size":5,"soft_fail":false,"target_va":5385687840,"va":5374771809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14107e840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11233060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374771440,"raw_hex":"e82467ab00","size":5,"soft_fail":false,"target_va":5386004544,"va":5374771479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8336367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374771440,"raw_hex":"e8ef337f00","size":5,"soft_fail":false,"target_va":5383108192,"va":5374771820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8271334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374771440,"raw_hex":"e8e6357e00","size":5,"soft_fail":false,"target_va":5383043136,"va":5374771797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::EventRegistry::register_event
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8367607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374771440,"raw_hex":"e8f7ad7f00","size":5,"soft_fail":false,"target_va":5383139696,"va":5374772084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] bevy_ecs::event::EventRegistry::register_event
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t8377084","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374771440,"raw_hex":"e9fcd27f00","size":5,"soft_fail":false,"target_va":5383149680,"va":5374772591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":431,"record_offset":112704,"parent_record_offset":112404,"ancestor_record_offset":112404,"inlinee":10733,"source_line":337,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":431,"record_offset":112620,"parent_record_offset":112404,"ancestor_record_offset":112404,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$4::in_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":431,"record_offset":112548,"parent_record_offset":112404,"ancestor_record_offset":112404,"inlinee":6385,"source_line":455,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::brush_sizes::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/brush_sizes.rs
//     evidence={"module":431,"record_offset":112676,"parent_record_offset":112404,"ancestor_record_offset":112404,"inlinee":24946,"source_line":56,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster_defs::PathRaster::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":431,"record_offset":112648,"parent_record_offset":112404,"ancestor_record_offset":112404,"inlinee":23641,"source_line":580,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":431,"record_offset":112728,"parent_record_offset":112704,"ancestor_record_offset":112404,"inlinee":10735,"source_line":304,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":431,"record_offset":112788,"parent_record_offset":112756,"ancestor_record_offset":112404,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/label.rs
//     evidence={"module":431,"record_offset":112588,"parent_record_offset":112548,"ancestor_record_offset":112404,"inlinee":6375,"source_line":129,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":431,"record_offset":114200,"parent_record_offset":114176,"ancestor_record_offset":112404,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":431,"record_offset":114176,"parent_record_offset":114152,"ancestor_record_offset":112404,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":431,"record_offset":113824,"parent_record_offset":113800,"ancestor_record_offset":112404,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":431,"record_offset":113872,"parent_record_offset":113848,"ancestor_record_offset":112404,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":431,"record_offset":113848,"parent_record_offset":113824,"ancestor_record_offset":112404,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":431,"record_offset":113964,"parent_record_offset":113900,"ancestor_record_offset":112404,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":431,"record_offset":114052,"parent_record_offset":114028,"ancestor_record_offset":112404,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":431,"record_offset":112756,"parent_record_offset":112728,"ancestor_record_offset":112404,"inlinee":10690,"source_line":1397,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":431,"record_offset":113800,"parent_record_offset":113776,"ancestor_record_offset":112404,"inlinee":10692,"source_line":1400,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$1
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":431,"record_offset":114152,"parent_record_offset":114128,"ancestor_record_offset":112404,"inlinee":10693,"source_line":1401,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":431,"record_offset":113508,"parent_record_offset":113484,"ancestor_record_offset":112404,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":431,"record_offset":113556,"parent_record_offset":113532,"ancestor_record_offset":112404,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":431,"record_offset":113484,"parent_record_offset":113460,"ancestor_record_offset":112404,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":431,"record_offset":112976,"parent_record_offset":112952,"ancestor_record_offset":112404,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":431,"record_offset":113060,"parent_record_offset":113036,"ancestor_record_offset":112404,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":431,"record_offset":113000,"parent_record_offset":112976,"ancestor_record_offset":112404,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":431,"record_offset":113172,"parent_record_offset":113148,"ancestor_record_offset":112404,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":431,"record_offset":113776,"parent_record_offset":112756,"ancestor_record_offset":112404,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":431,"record_offset":113988,"parent_record_offset":113964,"ancestor_record_offset":112404,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":431,"record_offset":113744,"parent_record_offset":112788,"ancestor_record_offset":112404,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":431,"record_offset":114028,"parent_record_offset":113848,"ancestor_record_offset":112404,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":431,"record_offset":113396,"parent_record_offset":113372,"ancestor_record_offset":112404,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":431,"record_offset":113316,"parent_record_offset":113292,"ancestor_record_offset":112404,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":431,"record_offset":113900,"parent_record_offset":113872,"ancestor_record_offset":112404,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":431,"record_offset":113936,"parent_record_offset":113900,"ancestor_record_offset":112404,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":431,"record_offset":113532,"parent_record_offset":113508,"ancestor_record_offset":112404,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":431,"record_offset":113460,"parent_record_offset":113436,"ancestor_record_offset":112404,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":431,"record_offset":112812,"parent_record_offset":112788,"ancestor_record_offset":112404,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":431,"record_offset":112836,"parent_record_offset":112812,"ancestor_record_offset":112404,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":431,"record_offset":113436,"parent_record_offset":113240,"ancestor_record_offset":112404,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":431,"record_offset":113348,"parent_record_offset":113264,"ancestor_record_offset":112404,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":431,"record_offset":113292,"parent_record_offset":113264,"ancestor_record_offset":112404,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":431,"record_offset":113692,"parent_record_offset":112912,"ancestor_record_offset":112404,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":431,"record_offset":113264,"parent_record_offset":113240,"ancestor_record_offset":112404,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":431,"record_offset":112888,"parent_record_offset":112864,"ancestor_record_offset":112404,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":431,"record_offset":112864,"parent_record_offset":112836,"ancestor_record_offset":112404,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":431,"record_offset":112912,"parent_record_offset":112888,"ancestor_record_offset":112404,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":431,"record_offset":113120,"parent_record_offset":113092,"ancestor_record_offset":112404,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":431,"record_offset":113148,"parent_record_offset":113120,"ancestor_record_offset":112404,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":431,"record_offset":113208,"parent_record_offset":113092,"ancestor_record_offset":112404,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":431,"record_offset":113092,"parent_record_offset":112912,"ancestor_record_offset":112404,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":431,"record_offset":113240,"parent_record_offset":112912,"ancestor_record_offset":112404,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":431,"record_offset":112952,"parent_record_offset":112912,"ancestor_record_offset":112404,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":431,"record_offset":113036,"parent_record_offset":112912,"ancestor_record_offset":112404,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":431,"record_offset":113608,"parent_record_offset":112912,"ancestor_record_offset":112404,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00431.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":431,"record_offset":114076,"parent_record_offset":114052,"ancestor_record_offset":112404,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00431.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::paths::PathPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$16::add_to_app<bevy_app::plugin::sealed::PluginMarker,feedback_reporter::FeedbackReporterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::menu_background::MenuBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::paths::PathPlugin,bevy_app::plugin::sealed::PluginMarker,system_water::WaterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::garden::GardenPlugin,bevy_app::plugin::sealed::PluginMarker,system_tree::TreePlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::prefab::PrefabPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::photo_mode::PhotoModePlugin>", "constructor": "bevy_app::plugin::sealed::impl$16::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "feedback_reporter::FeedbackReporterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::menu_background::MenuBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::paths::PathPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_water::WaterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::garden::GardenPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_tree::TreePlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::prefab::PrefabPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::vermeer::VermeerPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::photo_mode::PhotoModePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::systems::paths::PathPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::systems::paths::PathPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::systems::paths::PathPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::systems::paths::PathPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::systems::paths::PathPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::systems::paths::PathPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::systems::paths::PathPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::systems::paths::PathPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::systems::paths::PathPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::systems::paths::PathPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<country_core::systems::paths::PathPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["country_core::systems::paths::PathPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<country_core::systems::paths::PathPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["country_core::systems::paths::PathPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::EventReader<country_core::systems::history::history_process::NewReplayCommandEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::NewReplayCommandEvent>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::NewReplayCommandEvent>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::change_detection::ResMut<country_core::resources::steam::SteamInput>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::change_detection::ResMut<country_core::resources::steam::SteamInput>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::steam::SteamInput>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>,bevy_ecs::event::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>,bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>", "bevy_ecs::event::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>", "bevy_ecs::event::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>", "bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::history::replay_mode::HoldToFastReplay> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::history::replay_mode::HoldToFastReplay> >", "bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>", "bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::history::replay_mode::HoldToFastReplay> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::paths::impl$0::build
// 信息 ID: 4bc063feac1b8720903f4e66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
