// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/save/mod.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 288a201aea8f41ee69809650ffc800d3e617d3d8e96eeb1bc83fa9a9bac2e812
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 480
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
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
// 函数符号: country_core::systems::save::impl$0::add_save_load_systems
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":480,"record_offset":73032,"rva":6812496,"va":5375521616}
// 编译实例 RVAs: 0x67f350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400379d0
//     owner=unresolved_namespace; count=6
//     target_aliases=84 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6585516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375521616,"raw_hex":"e854839bff","size":5,"soft_fail":false,"target_va":5368936912,"va":5375522423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140037d90
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6583869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375521616,"raw_hex":"e8c3899bff","size":5,"soft_fail":false,"target_va":5368937872,"va":5375521736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140664690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-110108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375521616,"raw_hex":"e8e451feff","size":5,"soft_fail":false,"target_va":5375411856,"va":5375521959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140673c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-46989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375521616,"raw_hex":"e87348ffff","size":5,"soft_fail":false,"target_va":5375474704,"va":5375521688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d8cf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7396348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375521616,"raw_hex":"e8fcdb7000","size":5,"soft_fail":false,"target_va":5382918032,"va":5375521679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140da8ae0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7509794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375521616,"raw_hex":"e822977200","size":5,"soft_fail":false,"target_va":5383031520,"va":5375521721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411105c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11080093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375521616,"raw_hex":"e89d11a900","size":5,"soft_fail":false,"target_va":5386601920,"va":5375521822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7519623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375521616,"raw_hex":"e887bd7200","size":5,"soft_fail":false,"target_va":5383042000,"va":5375522372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::in_set
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6980267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375521616,"raw_hex":"e8ab826a00","size":5,"soft_fail":false,"target_va":5382501936,"va":5375521664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":480,"record_offset":73384,"parent_record_offset":73032,"ancestor_record_offset":73032,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$4::in_set
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":480,"record_offset":73180,"parent_record_offset":73032,"ancestor_record_offset":73032,"inlinee":6385,"source_line":455,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/label.rs
//     evidence={"module":480,"record_offset":73216,"parent_record_offset":73180,"ancestor_record_offset":73032,"inlinee":6375,"source_line":129,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::impl$0::add_save_load_events_and_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":480,"record_offset":73416,"rva":6813568,"va":5375522688}
// 编译实例 RVAs: 0x67f780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2773263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375522688,"raw_hex":"e8f1aed5ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375522954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27454332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375522688,"raw_hex":"e87ceba201","size":5,"soft_fail":false,"target_va":5402978144,"va":5375523807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7538826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375522688,"raw_hex":"e88a087300","size":5,"soft_fail":false,"target_va":5383061888,"va":5375523057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::EventRegistry::register_event
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t7600951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375522688,"raw_hex":"e837fb7300","size":5,"soft_fail":false,"target_va":5383123888,"va":5375522932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":480,"record_offset":73576,"parent_record_offset":73416,"ancestor_record_offset":73416,"inlinee":10733,"source_line":337,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::load_game::LoadGameManager::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/load_game.rs
//     evidence={"module":480,"record_offset":75116,"parent_record_offset":73416,"ancestor_record_offset":73416,"inlinee":27134,"source_line":53,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":480,"record_offset":75208,"parent_record_offset":75184,"ancestor_record_offset":73416,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":480,"record_offset":75232,"parent_record_offset":75208,"ancestor_record_offset":73416,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":480,"record_offset":75152,"parent_record_offset":75116,"ancestor_record_offset":73416,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":480,"record_offset":75184,"parent_record_offset":75152,"ancestor_record_offset":73416,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":480,"record_offset":73600,"parent_record_offset":73576,"ancestor_record_offset":73416,"inlinee":10735,"source_line":304,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":480,"record_offset":73660,"parent_record_offset":73628,"ancestor_record_offset":73416,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":480,"record_offset":75064,"parent_record_offset":75040,"ancestor_record_offset":73416,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":480,"record_offset":75040,"parent_record_offset":75016,"ancestor_record_offset":73416,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":480,"record_offset":74688,"parent_record_offset":74664,"ancestor_record_offset":73416,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":480,"record_offset":74736,"parent_record_offset":74712,"ancestor_record_offset":73416,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":480,"record_offset":74712,"parent_record_offset":74688,"ancestor_record_offset":73416,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":480,"record_offset":74828,"parent_record_offset":74764,"ancestor_record_offset":73416,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":480,"record_offset":74916,"parent_record_offset":74892,"ancestor_record_offset":73416,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":480,"record_offset":73628,"parent_record_offset":73600,"ancestor_record_offset":73416,"inlinee":10690,"source_line":1397,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":480,"record_offset":74664,"parent_record_offset":74640,"ancestor_record_offset":73416,"inlinee":10692,"source_line":1400,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$1
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":480,"record_offset":75016,"parent_record_offset":74992,"ancestor_record_offset":73416,"inlinee":10693,"source_line":1401,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":480,"record_offset":74372,"parent_record_offset":74348,"ancestor_record_offset":73416,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":480,"record_offset":74420,"parent_record_offset":74396,"ancestor_record_offset":73416,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":480,"record_offset":74348,"parent_record_offset":74324,"ancestor_record_offset":73416,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":480,"record_offset":73848,"parent_record_offset":73824,"ancestor_record_offset":73416,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":480,"record_offset":73932,"parent_record_offset":73908,"ancestor_record_offset":73416,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":480,"record_offset":73872,"parent_record_offset":73848,"ancestor_record_offset":73416,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":480,"record_offset":74036,"parent_record_offset":74012,"ancestor_record_offset":73416,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":480,"record_offset":74640,"parent_record_offset":73628,"ancestor_record_offset":73416,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":480,"record_offset":74852,"parent_record_offset":74828,"ancestor_record_offset":73416,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":480,"record_offset":74608,"parent_record_offset":73660,"ancestor_record_offset":73416,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":480,"record_offset":74892,"parent_record_offset":74712,"ancestor_record_offset":73416,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":480,"record_offset":74260,"parent_record_offset":74236,"ancestor_record_offset":73416,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":480,"record_offset":74180,"parent_record_offset":74156,"ancestor_record_offset":73416,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":480,"record_offset":75260,"parent_record_offset":75232,"ancestor_record_offset":73416,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":480,"record_offset":74764,"parent_record_offset":74736,"ancestor_record_offset":73416,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":480,"record_offset":74800,"parent_record_offset":74764,"ancestor_record_offset":73416,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":480,"record_offset":74396,"parent_record_offset":74372,"ancestor_record_offset":73416,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":480,"record_offset":74324,"parent_record_offset":74300,"ancestor_record_offset":73416,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":480,"record_offset":73684,"parent_record_offset":73660,"ancestor_record_offset":73416,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":480,"record_offset":73708,"parent_record_offset":73684,"ancestor_record_offset":73416,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":480,"record_offset":74300,"parent_record_offset":74104,"ancestor_record_offset":73416,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":480,"record_offset":74212,"parent_record_offset":74128,"ancestor_record_offset":73416,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":480,"record_offset":74156,"parent_record_offset":74128,"ancestor_record_offset":73416,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":480,"record_offset":74556,"parent_record_offset":73784,"ancestor_record_offset":73416,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":480,"record_offset":74128,"parent_record_offset":74104,"ancestor_record_offset":73416,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":480,"record_offset":73760,"parent_record_offset":73736,"ancestor_record_offset":73416,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":480,"record_offset":73736,"parent_record_offset":73708,"ancestor_record_offset":73416,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":480,"record_offset":73784,"parent_record_offset":73760,"ancestor_record_offset":73416,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":480,"record_offset":73988,"parent_record_offset":73964,"ancestor_record_offset":73416,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":480,"record_offset":74012,"parent_record_offset":73988,"ancestor_record_offset":73416,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":480,"record_offset":74072,"parent_record_offset":73964,"ancestor_record_offset":73416,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":480,"record_offset":73964,"parent_record_offset":73784,"ancestor_record_offset":73416,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":480,"record_offset":74104,"parent_record_offset":73784,"ancestor_record_offset":73416,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":480,"record_offset":73824,"parent_record_offset":73784,"ancestor_record_offset":73416,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":480,"record_offset":73908,"parent_record_offset":73784,"ancestor_record_offset":73416,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":480,"record_offset":74472,"parent_record_offset":73784,"ancestor_record_offset":73416,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":480,"record_offset":74940,"parent_record_offset":74916,"ancestor_record_offset":73416,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00480.jsonl.gz"}
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
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
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
// 文件级候选接口: 18; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>,bevy_ecs::event::EventReader<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::event::EventReader<country_core::resources::path_stroke::PathEraseEvent>,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventReader<country_core::systems::audio::stairs_audio::SuppressPathSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>", "bevy_ecs::event::EventReader<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::event::EventReader<country_core::resources::path_stroke::PathEraseEvent>", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::event::EventReader<country_core::systems::audio::stairs_audio::SuppressPathSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::path_stroke::PathEraseEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::stairs_audio::SuppressPathSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>,bevy_ecs::event::EventReader<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::event::EventReader<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>", "bevy_ecs::event::EventReader<country_core::systems::garden::GardenDrawEvent>", "bevy_ecs::event::EventReader<country_core::systems::garden::GardenEraseEvent>", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::garden::GardenDrawEvent>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::garden::GardenEraseEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>,bevy_ecs::event::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>", "bevy_ecs::event::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::event::EventReader<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::event::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>", "bevy_ecs::event::EventReader<country_core::resources::events::InitAdjWallHeightMode>", "bevy_ecs::event::EventReader<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::event::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::event::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::InitAdjWallHeightMode>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::audio::replay_audio::ReplayAudioState>,bevy_ecs::event::EventReader<country_core::systems::audio::replay_audio::PlayReplayAudio>,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::systems::audio::replay_audio::ReplayAudioState>", "bevy_ecs::event::EventReader<country_core::systems::audio::replay_audio::PlayReplayAudio>", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::replay_audio::ReplayAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::replay_audio::PlayReplayAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::trigger_audio::TriggerAudioState>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::trigger_audio::TriggerAudioState>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::trigger_audio::TriggerAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>,bevy_ecs::event::EventReader<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>", "bevy_ecs::event::EventReader<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::impl$0::add_save_load_events_and_resources
// 信息 ID: 4de600050375134cb4f01f3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3cf8e6 -> 0x67f780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::impl$0::add_save_load_systems
// 信息 ID: 9266b8cdc010b00c1d70ad88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3cf8be -> 0x67f350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
