// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f3a773aa3f6b0466dd57112d77b41ccda5d11bb929d965d74e43780b2138d992
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 322
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_app; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use toolbar; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":322,"record_offset":524,"rva":4473616,"va":5373182736}
// 编译实例 RVAs: 0x444310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400379d0
//     owner=unresolved_namespace; count=4
//     target_aliases=84 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4248674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182736,"raw_hex":"e89e2bbfff","size":5,"soft_fail":false,"target_va":5368936912,"va":5373185581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140037d90
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4247212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182736,"raw_hex":"e85431bfff","size":5,"soft_fail":false,"target_va":5368937872,"va":5373185079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a89cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6576494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182736,"raw_hex":"e86e596400","size":5,"soft_fail":false,"target_va":5379759280,"va":5373182781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6475954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182736,"raw_hex":"e8b2d06200","size":5,"soft_fail":false,"target_va":5379658784,"va":5373182825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6478005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182736,"raw_hex":"e8b5d86200","size":5,"soft_fail":false,"target_va":5379663392,"va":5373185382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::EventRegistry::register_event
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t6545943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182736,"raw_hex":"e817e26300","size":5,"soft_fail":false,"target_va":5379729008,"va":5373183060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t6514390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182736,"raw_hex":"e8d6666300","size":5,"soft_fail":false,"target_va":5379699456,"va":5373185061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t6581041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182736,"raw_hex":"e8316b6400","size":5,"soft_fail":false,"target_va":5379766080,"va":5373185034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::schedule::Schedules::add_systems
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6487709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182736,"raw_hex":"e89dfe6200","size":5,"soft_fail":false,"target_va":5379672736,"va":5373185022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::World::resource_mut::he5a472bf72332db6 (.llvm.16948530979910042347)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6479720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182736,"raw_hex":"e868df6200","size":5,"soft_fail":false,"target_va":5379664720,"va":5373184995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":322,"record_offset":672,"parent_record_offset":524,"ancestor_record_offset":524,"inlinee":10733,"source_line":337,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":322,"record_offset":14604,"parent_record_offset":524,"ancestor_record_offset":524,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$4::in_set
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":322,"record_offset":14724,"parent_record_offset":524,"ancestor_record_offset":524,"inlinee":6385,"source_line":455,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":322,"record_offset":644,"parent_record_offset":524,"ancestor_record_offset":524,"inlinee":17514,"source_line":51,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":322,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":524,"inlinee":10735,"source_line":304,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":322,"record_offset":14628,"parent_record_offset":14604,"ancestor_record_offset":524,"inlinee":13630,"source_line":180,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":322,"record_offset":14684,"parent_record_offset":14656,"ancestor_record_offset":524,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":322,"record_offset":14656,"parent_record_offset":14628,"ancestor_record_offset":524,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":322,"record_offset":756,"parent_record_offset":724,"ancestor_record_offset":524,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/label.rs
//     evidence={"module":322,"record_offset":14764,"parent_record_offset":14724,"ancestor_record_offset":524,"inlinee":6375,"source_line":129,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":322,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":524,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":322,"record_offset":2144,"parent_record_offset":2120,"ancestor_record_offset":524,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":322,"record_offset":1792,"parent_record_offset":1768,"ancestor_record_offset":524,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":322,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":524,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":322,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":524,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":322,"record_offset":1932,"parent_record_offset":1868,"ancestor_record_offset":524,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":322,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":524,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":322,"record_offset":724,"parent_record_offset":696,"ancestor_record_offset":524,"inlinee":10690,"source_line":1397,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":322,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":524,"inlinee":10692,"source_line":1400,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$1
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":322,"record_offset":2120,"parent_record_offset":2096,"ancestor_record_offset":524,"inlinee":10693,"source_line":1401,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":322,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":524,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":322,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":524,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":322,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":524,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":322,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":524,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":322,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":524,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":322,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":524,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":322,"record_offset":1140,"parent_record_offset":1116,"ancestor_record_offset":524,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":322,"record_offset":1744,"parent_record_offset":724,"ancestor_record_offset":524,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":322,"record_offset":1956,"parent_record_offset":1932,"ancestor_record_offset":524,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":322,"record_offset":1712,"parent_record_offset":756,"ancestor_record_offset":524,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":322,"record_offset":1996,"parent_record_offset":1816,"ancestor_record_offset":524,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":322,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":524,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":322,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":524,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":322,"record_offset":1868,"parent_record_offset":1840,"ancestor_record_offset":524,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":322,"record_offset":1904,"parent_record_offset":1868,"ancestor_record_offset":524,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":322,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":524,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":322,"record_offset":1428,"parent_record_offset":1404,"ancestor_record_offset":524,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":322,"record_offset":780,"parent_record_offset":756,"ancestor_record_offset":524,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":322,"record_offset":804,"parent_record_offset":780,"ancestor_record_offset":524,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":322,"record_offset":1404,"parent_record_offset":1208,"ancestor_record_offset":524,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":322,"record_offset":1316,"parent_record_offset":1232,"ancestor_record_offset":524,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":322,"record_offset":1260,"parent_record_offset":1232,"ancestor_record_offset":524,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":322,"record_offset":1660,"parent_record_offset":880,"ancestor_record_offset":524,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":322,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":524,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":322,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":524,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":322,"record_offset":832,"parent_record_offset":804,"ancestor_record_offset":524,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":322,"record_offset":880,"parent_record_offset":856,"ancestor_record_offset":524,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":322,"record_offset":1088,"parent_record_offset":1060,"ancestor_record_offset":524,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":322,"record_offset":1116,"parent_record_offset":1088,"ancestor_record_offset":524,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":322,"record_offset":1176,"parent_record_offset":1060,"ancestor_record_offset":524,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":322,"record_offset":1060,"parent_record_offset":880,"ancestor_record_offset":524,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":322,"record_offset":1208,"parent_record_offset":880,"ancestor_record_offset":524,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":322,"record_offset":920,"parent_record_offset":880,"ancestor_record_offset":524,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":322,"record_offset":1004,"parent_record_offset":880,"ancestor_record_offset":524,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":322,"record_offset":1576,"parent_record_offset":880,"ancestor_record_offset":524,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":322,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":524,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::impl$1::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":322,"record_offset":14996,"rva":4476624,"va":5373185744}
// 编译实例 RVAs: 0x444ed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400379d0
//     owner=unresolved_namespace; count=1
//     target_aliases=84 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4249084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373185744,"raw_hex":"e8042abfff","size":5,"soft_fail":false,"target_va":5368936912,"va":5373185991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140037d90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4247990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373185744,"raw_hex":"e84a2ebfff","size":5,"soft_fail":false,"target_va":5368937872,"va":5373185857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7c790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6518783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373185744,"raw_hex":"e8ff776300","size":5,"soft_fail":false,"target_va":5379704720,"va":5373185932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6477988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373185744,"raw_hex":"e8a4d86200","size":5,"soft_fail":false,"target_va":5379663936,"va":5373185943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6513609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373185744,"raw_hex":"e8c9636300","size":5,"soft_fail":false,"target_va":5379699456,"va":5373185842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6598282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373185744,"raw_hex":"e88aae6400","size":5,"soft_fail":false,"target_va":5379784096,"va":5373185809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::schedule::Schedules::add_systems
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6485283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373185744,"raw_hex":"e823f56200","size":5,"soft_fail":false,"target_va":5379671088,"va":5373185800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::World::resource_mut::he5a472bf72332db6 (.llvm.16948530979910042347)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6478935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373185744,"raw_hex":"e857dc6200","size":5,"soft_fail":false,"target_va":5379664720,"va":5373185780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":322,"record_offset":15108,"parent_record_offset":14996,"ancestor_record_offset":14996,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$4::in_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":322,"record_offset":15224,"parent_record_offset":14996,"ancestor_record_offset":14996,"inlinee":6385,"source_line":455,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":322,"record_offset":15132,"parent_record_offset":15108,"ancestor_record_offset":14996,"inlinee":13630,"source_line":180,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":322,"record_offset":15184,"parent_record_offset":15156,"ancestor_record_offset":14996,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":322,"record_offset":15156,"parent_record_offset":15132,"ancestor_record_offset":14996,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00322.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/label.rs
//     evidence={"module":322,"record_offset":15260,"parent_record_offset":15224,"ancestor_record_offset":14996,"inlinee":6375,"source_line":129,"archive":"_pdb/A/modules/00322.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::ToolbarPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::ToolbarPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::ToolbarPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<toolbar::ToolbarPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["toolbar::ToolbarPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<toolbar::ToolbarPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["toolbar::ToolbarPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<toolbar::ToolbarPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["toolbar::ToolbarPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<toolbar::ToolbarPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["toolbar::ToolbarPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<toolbar::ToolbarPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["toolbar::ToolbarPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<toolbar::ToolbarPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["toolbar::ToolbarPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$14::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$14::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::Ui2dPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::Ui2dPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::Ui2dPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<toolbar::Ui2dPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["toolbar::Ui2dPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<toolbar::Ui2dPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["toolbar::Ui2dPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<toolbar::Ui2dPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["toolbar::Ui2dPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::Ui2dPlugin>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::Ui2dPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<toolbar::Ui2dPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["toolbar::Ui2dPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<toolbar::Ui2dPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["toolbar::Ui2dPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$14::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$14::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: toolbar::impl$0::build
// 信息 ID: a13bade6d9968d166729786b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::impl$1::build
// 信息 ID: 39236c16d61c978bb98a08c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
