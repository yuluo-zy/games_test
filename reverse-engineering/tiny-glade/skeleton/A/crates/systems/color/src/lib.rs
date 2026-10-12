// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/color/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 7f562fc6b2810167d18b8944317fbe675e66a81078d315411c9f8f4177410967
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 294
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::impl$0::add_color
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":294,"record_offset":532,"rva":4260960,"va":5372970080}
// 编译实例 RVAs: 0x410460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400379d0
//     owner=unresolved_namespace; count=2
//     target_aliases=84 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4035602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372970080,"raw_hex":"e8ee6bc2ff","size":5,"soft_fail":false,"target_va":5368936912,"va":5372972509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140037d90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4034261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372970080,"raw_hex":"e82b71c2ff","size":5,"soft_fail":false,"target_va":5368937872,"va":5372972128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14052fbd0
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1175066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372970080,"raw_hex":"e81aee1100","size":5,"soft_fail":false,"target_va":5374147536,"va":5372972465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409ae690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5888603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372970080,"raw_hex":"e85bda5900","size":5,"soft_fail":false,"target_va":5378860688,"va":5372972080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30620983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372970080,"raw_hex":"e8373dd301","size":5,"soft_fail":false,"target_va":5403593360,"va":5372972372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::EventRegistry::register_event
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t5870919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372970080,"raw_hex":"e847955900","size":5,"soft_fail":false,"target_va":5378841264,"va":5372970340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5896010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372970080,"raw_hex":"e84af75900","size":5,"soft_fail":false,"target_va":5378868128,"va":5372972113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::in_set
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5883364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372970080,"raw_hex":"e8e4c55900","size":5,"soft_fail":false,"target_va":5378855440,"va":5372972071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::schedule::Schedules::add_systems
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5877049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372970080,"raw_hex":"e839ad5900","size":5,"soft_fail":false,"target_va":5378849104,"va":5372972050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::World::resource_mut::h466ce4295ac5e28c (.llvm.12598775437230246011)
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5872043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372970080,"raw_hex":"e8ab995900","size":5,"soft_fail":false,"target_va":5378844080,"va":5372972032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":294,"record_offset":656,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":10733,"source_line":337,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":294,"record_offset":13040,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$4::in_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":294,"record_offset":13160,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":6385,"source_line":455,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":294,"record_offset":680,"parent_record_offset":656,"ancestor_record_offset":532,"inlinee":10735,"source_line":304,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_systems
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":294,"record_offset":13064,"parent_record_offset":13040,"ancestor_record_offset":532,"inlinee":13630,"source_line":180,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":294,"record_offset":13120,"parent_record_offset":13092,"ancestor_record_offset":532,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":294,"record_offset":13092,"parent_record_offset":13064,"ancestor_record_offset":532,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":294,"record_offset":740,"parent_record_offset":708,"ancestor_record_offset":532,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/label.rs
//     evidence={"module":294,"record_offset":13196,"parent_record_offset":13160,"ancestor_record_offset":532,"inlinee":6375,"source_line":129,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":294,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":532,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":294,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":532,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":294,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":532,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":294,"record_offset":1824,"parent_record_offset":1800,"ancestor_record_offset":532,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":294,"record_offset":1800,"parent_record_offset":1776,"ancestor_record_offset":532,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":294,"record_offset":1916,"parent_record_offset":1852,"ancestor_record_offset":532,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":294,"record_offset":2004,"parent_record_offset":1980,"ancestor_record_offset":532,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":294,"record_offset":708,"parent_record_offset":680,"ancestor_record_offset":532,"inlinee":10690,"source_line":1397,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":294,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":532,"inlinee":10692,"source_line":1400,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$1
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":294,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":532,"inlinee":10693,"source_line":1401,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":294,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":532,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":294,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":532,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":294,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":532,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":294,"record_offset":928,"parent_record_offset":904,"ancestor_record_offset":532,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":294,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":532,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":294,"record_offset":952,"parent_record_offset":928,"ancestor_record_offset":532,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":294,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":532,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":294,"record_offset":1728,"parent_record_offset":708,"ancestor_record_offset":532,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":294,"record_offset":1940,"parent_record_offset":1916,"ancestor_record_offset":532,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":294,"record_offset":1696,"parent_record_offset":740,"ancestor_record_offset":532,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":294,"record_offset":1980,"parent_record_offset":1800,"ancestor_record_offset":532,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":294,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":532,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":294,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":532,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":294,"record_offset":1852,"parent_record_offset":1824,"ancestor_record_offset":532,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":294,"record_offset":1888,"parent_record_offset":1852,"ancestor_record_offset":532,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":294,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":532,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":294,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":532,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":294,"record_offset":764,"parent_record_offset":740,"ancestor_record_offset":532,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":294,"record_offset":788,"parent_record_offset":764,"ancestor_record_offset":532,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":294,"record_offset":1388,"parent_record_offset":1192,"ancestor_record_offset":532,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":294,"record_offset":1300,"parent_record_offset":1216,"ancestor_record_offset":532,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":294,"record_offset":1244,"parent_record_offset":1216,"ancestor_record_offset":532,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":294,"record_offset":1644,"parent_record_offset":864,"ancestor_record_offset":532,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":294,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":532,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":294,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":532,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":294,"record_offset":816,"parent_record_offset":788,"ancestor_record_offset":532,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":294,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":532,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":294,"record_offset":1072,"parent_record_offset":1044,"ancestor_record_offset":532,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":294,"record_offset":1100,"parent_record_offset":1072,"ancestor_record_offset":532,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":294,"record_offset":1160,"parent_record_offset":1044,"ancestor_record_offset":532,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":294,"record_offset":1044,"parent_record_offset":864,"ancestor_record_offset":532,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":294,"record_offset":1192,"parent_record_offset":864,"ancestor_record_offset":532,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":294,"record_offset":904,"parent_record_offset":864,"ancestor_record_offset":532,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":294,"record_offset":988,"parent_record_offset":864,"ancestor_record_offset":532,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":294,"record_offset":1560,"parent_record_offset":864,"ancestor_record_offset":532,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00294.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":294,"record_offset":2028,"parent_record_offset":2004,"ancestor_record_offset":532,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00294.jsonl.gz"}
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
// 函数: system_color::impl$0::add_color
// 信息 ID: 1a77419ccd9d411d3a3b9b9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3d1bae -> 0x410460 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
