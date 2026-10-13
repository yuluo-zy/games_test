// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/menu/mod.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 12db93dbe8eff825896f1fdac3fd8c0c8e247e13e803576b3ea0218cf8f83b24
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 323, 2499
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
// use toolbar; // namespace candidate
//   package identities: []
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"PDB_registry_path_only","version":"1.1.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":323,"record_offset":524,"rva":4476896,"va":5373186016}
// 编译实例 RVAs: 0x444fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400379d0
//     owner=unresolved_namespace; count=11
//     target_aliases=84 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4251554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373186016,"raw_hex":"e85e20bfff","size":5,"soft_fail":false,"target_va":5368936912,"va":5373188461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140037d90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4249913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373186016,"raw_hex":"e8c726bfff","size":5,"soft_fail":false,"target_va":5368937872,"va":5373187780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-436898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373186016,"raw_hex":"e85e55f9ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5373186589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7cb50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6517969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373186016,"raw_hex":"e8d1746300","size":5,"soft_fail":false,"target_va":5379705680,"va":5373187706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a92c00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6609109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373186016,"raw_hex":"e8d5d86400","size":5,"soft_fail":false,"target_va":5379795968,"va":5373186854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a938c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6612359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373186016,"raw_hex":"e887e56400","size":5,"soft_fail":false,"target_va":5379799232,"va":5373186868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a94840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6616313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373186016,"raw_hex":"e8f9f46400","size":5,"soft_fail":false,"target_va":5379803200,"va":5373186882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29789725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373186016,"raw_hex":"e81d8ec601","size":5,"soft_fail":false,"target_va":5402978144,"va":5373188414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t6474054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373186016,"raw_hex":"e846c96200","size":5,"soft_fail":false,"target_va":5379660128,"va":5373186069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6476214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373186016,"raw_hex":"e8b6d16200","size":5,"soft_fail":false,"target_va":5379663936,"va":5373187717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::EventRegistry::register_event
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6539207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373186016,"raw_hex":"e8c7c76300","size":5,"soft_fail":false,"target_va":5379727344,"va":5373188132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6511689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373186016,"raw_hex":"e8495c6300","size":5,"soft_fail":false,"target_va":5379699456,"va":5373187762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::run_if
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t6607658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373186016,"raw_hex":"e82ad36400","size":5,"soft_fail":false,"target_va":5379793824,"va":5373186161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] bevy_ecs::event::EventRegistry::register_event
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t6541436","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5373186016,"raw_hex":"e97cd06300","size":5,"soft_fail":false,"target_va":5379729840,"va":5373188399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":323,"record_offset":1168,"parent_record_offset":524,"ancestor_record_offset":524,"inlinee":10733,"source_line":337,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":323,"record_offset":1040,"parent_record_offset":524,"ancestor_record_offset":524,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":323,"record_offset":736,"parent_record_offset":524,"ancestor_record_offset":524,"inlinee":6399,"source_line":444,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$4::in_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":323,"record_offset":1068,"parent_record_offset":524,"ancestor_record_offset":524,"inlinee":6385,"source_line":455,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::impl$16::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":323,"record_offset":660,"parent_record_offset":524,"ancestor_record_offset":524,"inlinee":17522,"source_line":92,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::impl$20::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":323,"record_offset":688,"parent_record_offset":524,"ancestor_record_offset":524,"inlinee":17524,"source_line":112,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::impl$23::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":323,"record_offset":712,"parent_record_offset":524,"ancestor_record_offset":524,"inlinee":17526,"source_line":118,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":323,"record_offset":880,"parent_record_offset":856,"ancestor_record_offset":524,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":323,"record_offset":904,"parent_record_offset":880,"ancestor_record_offset":524,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":323,"record_offset":824,"parent_record_offset":776,"ancestor_record_offset":524,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":323,"record_offset":856,"parent_record_offset":824,"ancestor_record_offset":524,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":323,"record_offset":1192,"parent_record_offset":1168,"ancestor_record_offset":524,"inlinee":10735,"source_line":304,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":323,"record_offset":1252,"parent_record_offset":1220,"ancestor_record_offset":524,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::NodeConfigs::chain_ignore_deferred_inner
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":323,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":524,"inlinee":6384,"source_line":246,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::impl$10::into_configs
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":323,"record_offset":776,"parent_record_offset":736,"ancestor_record_offset":524,"inlinee":6381,"source_line":530,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::impl$4::chain_ignore_deferred
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":323,"record_offset":980,"parent_record_offset":736,"ancestor_record_offset":524,"inlinee":6383,"source_line":515,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/label.rs
//     evidence={"module":323,"record_offset":1108,"parent_record_offset":1068,"ancestor_record_offset":524,"inlinee":6375,"source_line":129,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":323,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":524,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":323,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":524,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":323,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":524,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":323,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":524,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":323,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":524,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":323,"record_offset":2428,"parent_record_offset":2364,"ancestor_record_offset":524,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":323,"record_offset":2516,"parent_record_offset":2492,"ancestor_record_offset":524,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":323,"record_offset":1220,"parent_record_offset":1192,"ancestor_record_offset":524,"inlinee":10690,"source_line":1397,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":323,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":524,"inlinee":10692,"source_line":1400,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$1
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":323,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":524,"inlinee":10693,"source_line":1401,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":323,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":524,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":323,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":524,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":323,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":524,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":323,"record_offset":1440,"parent_record_offset":1416,"ancestor_record_offset":524,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":323,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":524,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":323,"record_offset":1464,"parent_record_offset":1440,"ancestor_record_offset":524,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":323,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":524,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":323,"record_offset":2240,"parent_record_offset":1220,"ancestor_record_offset":524,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":323,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":524,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":323,"record_offset":2208,"parent_record_offset":1252,"ancestor_record_offset":524,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":323,"record_offset":2492,"parent_record_offset":2312,"ancestor_record_offset":524,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":323,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":524,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":323,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":524,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":323,"record_offset":932,"parent_record_offset":904,"ancestor_record_offset":524,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":323,"record_offset":2364,"parent_record_offset":2336,"ancestor_record_offset":524,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":323,"record_offset":2400,"parent_record_offset":2364,"ancestor_record_offset":524,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":323,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":524,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":323,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":524,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":323,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":524,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":323,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":524,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":323,"record_offset":1900,"parent_record_offset":1704,"ancestor_record_offset":524,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":323,"record_offset":1812,"parent_record_offset":1728,"ancestor_record_offset":524,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":323,"record_offset":1756,"parent_record_offset":1728,"ancestor_record_offset":524,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":323,"record_offset":2156,"parent_record_offset":1376,"ancestor_record_offset":524,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":323,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":524,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":323,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":524,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":323,"record_offset":1328,"parent_record_offset":1300,"ancestor_record_offset":524,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":323,"record_offset":1376,"parent_record_offset":1352,"ancestor_record_offset":524,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":323,"record_offset":1584,"parent_record_offset":1556,"ancestor_record_offset":524,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":323,"record_offset":1612,"parent_record_offset":1584,"ancestor_record_offset":524,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":323,"record_offset":1672,"parent_record_offset":1556,"ancestor_record_offset":524,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":323,"record_offset":1556,"parent_record_offset":1376,"ancestor_record_offset":524,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":323,"record_offset":1704,"parent_record_offset":1376,"ancestor_record_offset":524,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":323,"record_offset":1416,"parent_record_offset":1376,"ancestor_record_offset":524,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":323,"record_offset":1500,"parent_record_offset":1376,"ancestor_record_offset":524,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":323,"record_offset":2072,"parent_record_offset":1376,"ancestor_record_offset":524,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":323,"record_offset":2540,"parent_record_offset":2516,"ancestor_record_offset":524,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::impl$1::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":323,"record_offset":4272,"rva":4479808,"va":5373188928}
// 编译实例 RVAs: 0x445b40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6474963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373188928,"raw_hex":"e8d3cc6200","size":5,"soft_fail":false,"target_va":5379663936,"va":5373188968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::in_set
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6626622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373188928,"raw_hex":"e83e1d6500","size":5,"soft_fail":false,"target_va":5379815584,"va":5373188957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":323,"record_offset":4384,"parent_record_offset":4272,"ancestor_record_offset":4272,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::advance_menu_state
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":323,"record_offset":4416,"rva":4479872,"va":5373188992}
// 编译实例 RVAs: 0x445b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d06f0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1529546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373188992,"raw_hex":"e836a9e8ff","size":5,"soft_fail":false,"target_va":5371660016,"va":5373189557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056d1e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1209541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373188992,"raw_hex":"e8c5741200","size":5,"soft_fail":false,"target_va":5374398944,"va":5373189398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1884201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373188992,"raw_hex":"e8d73fe3ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5373189252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":323,"record_offset":4556,"parent_record_offset":4416,"ancestor_record_offset":4416,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send_default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":323,"record_offset":4876,"parent_record_offset":4416,"ancestor_record_offset":4416,"inlinee":7452,"source_line":641,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":323,"record_offset":5260,"parent_record_offset":4416,"ancestor_record_offset":4416,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":323,"record_offset":4616,"parent_record_offset":4416,"ancestor_record_offset":4416,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":323,"record_offset":5556,"parent_record_offset":4416,"ancestor_record_offset":4416,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::has_only_one_session
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/mod.rs
//     evidence={"module":323,"record_offset":4732,"parent_record_offset":4416,"ancestor_record_offset":4416,"inlinee":17528,"source_line":110,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":323,"record_offset":5064,"parent_record_offset":5040,"ancestor_record_offset":4416,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":323,"record_offset":5668,"parent_record_offset":5632,"ancestor_record_offset":4416,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":323,"record_offset":5632,"parent_record_offset":5604,"ancestor_record_offset":4416,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":323,"record_offset":5040,"parent_record_offset":5000,"ancestor_record_offset":4416,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":323,"record_offset":5000,"parent_record_offset":4964,"ancestor_record_offset":4416,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":323,"record_offset":4584,"parent_record_offset":4556,"ancestor_record_offset":4416,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":323,"record_offset":4900,"parent_record_offset":4876,"ancestor_record_offset":4416,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":323,"record_offset":4964,"parent_record_offset":4900,"ancestor_record_offset":4416,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":323,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":4416,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":323,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":4416,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":323,"record_offset":4640,"parent_record_offset":4616,"ancestor_record_offset":4416,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":323,"record_offset":5580,"parent_record_offset":5556,"ancestor_record_offset":4416,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":323,"record_offset":5700,"parent_record_offset":5668,"ancestor_record_offset":4416,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":323,"record_offset":4668,"parent_record_offset":4640,"ancestor_record_offset":4416,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":323,"record_offset":4696,"parent_record_offset":4640,"ancestor_record_offset":4416,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":323,"record_offset":5728,"parent_record_offset":5668,"ancestor_record_offset":4416,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":323,"record_offset":5760,"parent_record_offset":5632,"ancestor_record_offset":4416,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$21::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":323,"record_offset":4760,"parent_record_offset":4732,"ancestor_record_offset":4416,"inlinee":17530,"source_line":114,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$19::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.1.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/uuid-1.1.2/src/lib.rs
//     evidence={"module":323,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":4416,"inlinee":13086,"source_line":397,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::has_only_one_session
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4478
// 首条位置: {"module":323,"inlinee":17528}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::session::impl$21::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":323,"record_offset":4760,"parent_record_offset":4732,"ancestor_record_offset":4732,"inlinee":17530,"source_line":114,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":323,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":4732,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":323,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":4732,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00323.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$19::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"uuid","version":"1.1.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/uuid-1.1.2/src/lib.rs
//     evidence={"module":323,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":4732,"inlinee":13086,"source_line":397,"archive":"_pdb/A/modules/00323.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::MenuFlowPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::menu::MenuFlowPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<toolbar::menu::MenuFlowPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<toolbar::menu::MenuFlowPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<toolbar::menu::MenuFlowPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::menu::MenuFlowPlugin>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<toolbar::menu::MenuFlowPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<toolbar::menu::MenuFlowPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<toolbar::menu::MenuFlowPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$14::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$14::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::SplashScreenPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$9::add_to_app<bevy_app::plugin::sealed::PluginMarker,country_core::startup::MinimalStartupPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::logo_animation::AnimatedLogo,bevy_app::plugin::sealed::PluginMarker,country_core::resources::loading_screen::LoadingScreen,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::SplashScreenPlugin>", "constructor": "bevy_app::plugin::sealed::impl$9::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "country_core::startup::MinimalStartupPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::logo_animation::AnimatedLogo", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::loading_screen::LoadingScreen", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::vermeer::VermeerPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::menu::SplashScreenPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<toolbar::menu::SplashScreenPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<toolbar::menu::SplashScreenPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<toolbar::menu::SplashScreenPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<toolbar::menu::SplashScreenPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<toolbar::menu::SplashScreenPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>", "bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>

// ------------------------------------------------------------------
// 函数: toolbar::menu::advance_menu_state
// 信息 ID: b8cafc9d2025cdaea0e6b0f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f0df316716412a61e39f7a95: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>)> @ 0xa97094 -> 0x445b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>)> > @ 0x15d95a0 -> 0x445b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::has_only_one_session
// 信息 ID: 226110527bff2b49a2fd315d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::advance_menu_state; module=323, inline_record=4732; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::new_session::ui; module=2499, inline_record=1884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::impl$0::build
// 信息 ID: abb1a318f3dfc261528afc33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::impl$1::build
// 信息 ID: 8f4f35e31316b76d51c6af74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
