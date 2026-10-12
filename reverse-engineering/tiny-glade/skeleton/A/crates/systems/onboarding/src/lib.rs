// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/onboarding/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d3365b295403910556fa017ebe7e09b29e9fa4bc0e044971cd98043ebe54b21a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 293
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_onboarding::impl$0::add_onboarding
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":293,"record_offset":536,"rva":4256416,"va":5372965536}
// 编译实例 RVAs: 0x40f2a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400379d0
//     owner=unresolved_namespace; count=6
//     target_aliases=84 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4032914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372965536,"raw_hex":"e86e76c2ff","size":5,"soft_fail":false,"target_va":5368936912,"va":5372969821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140037d90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4028355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372965536,"raw_hex":"e83d88c2ff","size":5,"soft_fail":false,"target_va":5368937872,"va":5372966222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-216259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372965536,"raw_hex":"e83db3fcff","size":5,"soft_fail":false,"target_va":5372749696,"va":5372965950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30008359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372965536,"raw_hex":"e827e4c901","size":5,"soft_fail":false,"target_va":5402978144,"va":5372969780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::EventRegistry::register_event
//     owner=unresolved_namespace; count=13
//     evidence={"assembly":"\tcallq\t5845687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372965536,"raw_hex":"e8b7325900","size":5,"soft_fail":false,"target_va":5378812560,"va":5372966868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5839327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372965536,"raw_hex":"e8df195900","size":5,"soft_fail":false,"target_va":5378805536,"va":5372966204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5815475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372965536,"raw_hex":"e8b3bc5800","size":5,"soft_fail":false,"target_va":5378781088,"va":5372965608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::in_set
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5835997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372965536,"raw_hex":"e8dd0c5900","size":5,"soft_fail":false,"target_va":5378802496,"va":5372966494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::schedule::Schedules::add_systems
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t5808543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372965536,"raw_hex":"e89fa15800","size":5,"soft_fail":false,"target_va":5378774144,"va":5372965596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::World::resource_mut::hb58792d1b7223a34 (.llvm.1560296113162983900)
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t5803537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372965536,"raw_hex":"e8118e5800","size":5,"soft_fail":false,"target_va":5378769120,"va":5372965578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":293,"record_offset":1444,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":10733,"source_line":337,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":293,"record_offset":676,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":293,"record_offset":792,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":6399,"source_line":444,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$4::in_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":293,"record_offset":1092,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":6385,"source_line":455,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":293,"record_offset":932,"parent_record_offset":908,"ancestor_record_offset":536,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":293,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":536,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":293,"record_offset":876,"parent_record_offset":828,"ancestor_record_offset":536,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":293,"record_offset":908,"parent_record_offset":876,"ancestor_record_offset":536,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":293,"record_offset":1468,"parent_record_offset":1444,"ancestor_record_offset":536,"inlinee":10735,"source_line":304,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_systems
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":293,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":536,"inlinee":13630,"source_line":180,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":293,"record_offset":752,"parent_record_offset":724,"ancestor_record_offset":536,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":293,"record_offset":724,"parent_record_offset":700,"ancestor_record_offset":536,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":293,"record_offset":1528,"parent_record_offset":1496,"ancestor_record_offset":536,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::NodeConfigs::chain_ignore_deferred_inner
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":293,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":536,"inlinee":6384,"source_line":246,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::impl$10::into_configs
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":293,"record_offset":828,"parent_record_offset":792,"ancestor_record_offset":536,"inlinee":6381,"source_line":530,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::impl$4::chain_ignore_deferred
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":293,"record_offset":1032,"parent_record_offset":792,"ancestor_record_offset":536,"inlinee":6383,"source_line":515,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/label.rs
//     evidence={"module":293,"record_offset":1132,"parent_record_offset":1092,"ancestor_record_offset":536,"inlinee":6375,"source_line":129,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":293,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":536,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":293,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":536,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":293,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":536,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":293,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":536,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":293,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":536,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":293,"record_offset":2704,"parent_record_offset":2640,"ancestor_record_offset":536,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":293,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":536,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":293,"record_offset":1496,"parent_record_offset":1468,"ancestor_record_offset":536,"inlinee":10690,"source_line":1397,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":293,"record_offset":2540,"parent_record_offset":2516,"ancestor_record_offset":536,"inlinee":10692,"source_line":1400,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$1
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":293,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":536,"inlinee":10693,"source_line":1401,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":293,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":536,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":293,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":536,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":293,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":536,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":293,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":536,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":293,"record_offset":1800,"parent_record_offset":1776,"ancestor_record_offset":536,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":293,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":536,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":293,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":536,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":293,"record_offset":2516,"parent_record_offset":1496,"ancestor_record_offset":536,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":293,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":536,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":293,"record_offset":2484,"parent_record_offset":1528,"ancestor_record_offset":536,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":293,"record_offset":2768,"parent_record_offset":2588,"ancestor_record_offset":536,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":293,"record_offset":2136,"parent_record_offset":2112,"ancestor_record_offset":536,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":293,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":536,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":293,"record_offset":984,"parent_record_offset":956,"ancestor_record_offset":536,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":293,"record_offset":2640,"parent_record_offset":2612,"ancestor_record_offset":536,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":293,"record_offset":2676,"parent_record_offset":2640,"ancestor_record_offset":536,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":293,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":536,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":293,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":536,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":293,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":536,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":293,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":536,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":293,"record_offset":2176,"parent_record_offset":1980,"ancestor_record_offset":536,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":293,"record_offset":2088,"parent_record_offset":2004,"ancestor_record_offset":536,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":293,"record_offset":2032,"parent_record_offset":2004,"ancestor_record_offset":536,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":293,"record_offset":2432,"parent_record_offset":1652,"ancestor_record_offset":536,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":293,"record_offset":2004,"parent_record_offset":1980,"ancestor_record_offset":536,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":293,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":536,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":293,"record_offset":1604,"parent_record_offset":1576,"ancestor_record_offset":536,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":293,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":536,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":293,"record_offset":1860,"parent_record_offset":1832,"ancestor_record_offset":536,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":293,"record_offset":1888,"parent_record_offset":1860,"ancestor_record_offset":536,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":293,"record_offset":1948,"parent_record_offset":1832,"ancestor_record_offset":536,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":293,"record_offset":1832,"parent_record_offset":1652,"ancestor_record_offset":536,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":293,"record_offset":1980,"parent_record_offset":1652,"ancestor_record_offset":536,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":293,"record_offset":1692,"parent_record_offset":1652,"ancestor_record_offset":536,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=26
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":293,"record_offset":1776,"parent_record_offset":1652,"ancestor_record_offset":536,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":293,"record_offset":2348,"parent_record_offset":1652,"ancestor_record_offset":536,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00293.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":293,"record_offset":2816,"parent_record_offset":2792,"ancestor_record_offset":536,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00293.jsonl.gz"}
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
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
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
// 函数: system_onboarding::impl$0::add_onboarding
// 信息 ID: cbbaee102b23d0b6bc47689f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3d1bfe -> 0x40f2a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
