// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/gui/settings.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: c4b92f00b0d9aba7e5626b27d8962817bedda611d5780392718c6476ccc14a51
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: settings.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 15, 21, 58, 73, 122, 170, 194, 214, 220
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_input; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"}]
// use toolbar; // namespace candidate
//   package identities: []
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"PDB_registry_path_only","version":"1.1.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":170,"record_offset":232,"rva":2951648,"va":5371660768}
// 编译实例 RVAs: 0x2d09e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140291960
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-258255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371660768,"raw_hex":"e8310ffcff","size":5,"soft_fail":false,"target_va":5371402592,"va":5371660842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-272775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371660768,"raw_hex":"e879d6fbff","size":5,"soft_fail":false,"target_va":5371388032,"va":5371660802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] bevy_ecs::event::EventRegistry::register_event
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-1026704","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5371660768,"raw_hex":"e97055f0ff","size":5,"soft_fail":false,"target_va":5370634400,"va":5371661099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":170,"record_offset":412,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":10733,"source_line":337,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::impl$12::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":170,"record_offset":388,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":12050,"source_line":56,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::settings::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":170,"record_offset":364,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":12048,"source_line":58,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":170,"record_offset":440,"parent_record_offset":412,"ancestor_record_offset":232,"inlinee":10735,"source_line":304,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":170,"record_offset":500,"parent_record_offset":468,"ancestor_record_offset":232,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":170,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":232,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":170,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":232,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":170,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":232,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":170,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":232,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":170,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":232,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":170,"record_offset":1676,"parent_record_offset":1612,"ancestor_record_offset":232,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":170,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":232,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":170,"record_offset":468,"parent_record_offset":440,"ancestor_record_offset":232,"inlinee":10690,"source_line":1397,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":170,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":232,"inlinee":10692,"source_line":1400,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$1
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":170,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":232,"inlinee":10693,"source_line":1401,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":170,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":232,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":232,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":232,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":170,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":232,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":170,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":232,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":170,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":232,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":170,"record_offset":884,"parent_record_offset":860,"ancestor_record_offset":232,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":170,"record_offset":1488,"parent_record_offset":468,"ancestor_record_offset":232,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":170,"record_offset":1700,"parent_record_offset":1676,"ancestor_record_offset":232,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":170,"record_offset":1456,"parent_record_offset":500,"ancestor_record_offset":232,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":170,"record_offset":1740,"parent_record_offset":1560,"ancestor_record_offset":232,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":170,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":232,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":170,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":232,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":170,"record_offset":1612,"parent_record_offset":1584,"ancestor_record_offset":232,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":170,"record_offset":1648,"parent_record_offset":1612,"ancestor_record_offset":232,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":170,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":232,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":170,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":232,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":170,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":232,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":170,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":232,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":170,"record_offset":1148,"parent_record_offset":952,"ancestor_record_offset":232,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":170,"record_offset":1060,"parent_record_offset":976,"ancestor_record_offset":232,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":170,"record_offset":1004,"parent_record_offset":976,"ancestor_record_offset":232,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":170,"record_offset":1404,"parent_record_offset":624,"ancestor_record_offset":232,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":170,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":232,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":170,"record_offset":600,"parent_record_offset":576,"ancestor_record_offset":232,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":170,"record_offset":576,"parent_record_offset":548,"ancestor_record_offset":232,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":170,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":232,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":170,"record_offset":832,"parent_record_offset":804,"ancestor_record_offset":232,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":170,"record_offset":860,"parent_record_offset":832,"ancestor_record_offset":232,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":170,"record_offset":920,"parent_record_offset":804,"ancestor_record_offset":232,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":170,"record_offset":804,"parent_record_offset":624,"ancestor_record_offset":232,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":170,"record_offset":952,"parent_record_offset":624,"ancestor_record_offset":232,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":170,"record_offset":664,"parent_record_offset":624,"ancestor_record_offset":232,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":170,"record_offset":748,"parent_record_offset":624,"ancestor_record_offset":232,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":170,"record_offset":1320,"parent_record_offset":624,"ancestor_record_offset":232,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":170,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":232,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::impl$7::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2F10
// 首条位置: {"module":170,"inlinee":12048}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::LastAppliedVideoSettings::populate_empty_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2F1B
// 首条位置: {"module":170,"inlinee":12059}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":170,"record_offset":3240,"parent_record_offset":3204,"ancestor_record_offset":3204,"inlinee":5289,"source_line":1660,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":170,"record_offset":3436,"parent_record_offset":3368,"ancestor_record_offset":3204,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":170,"record_offset":3468,"parent_record_offset":3436,"ancestor_record_offset":3204,"inlinee":10713,"source_line":29,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":170,"record_offset":3368,"parent_record_offset":3304,"ancestor_record_offset":3204,"inlinee":10709,"source_line":43,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$24::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":170,"record_offset":3304,"parent_record_offset":3276,"ancestor_record_offset":3204,"inlinee":10707,"source_line":75,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":170,"record_offset":3408,"parent_record_offset":3368,"ancestor_record_offset":3204,"inlinee":10711,"source_line":13,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":170,"record_offset":3580,"parent_record_offset":3304,"ancestor_record_offset":3204,"inlinee":10594,"source_line":87,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::impl$1::populate_empty_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":170,"record_offset":3276,"parent_record_offset":3240,"ancestor_record_offset":3204,"inlinee":12061,"source_line":63,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::impl$1::populate_empty_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2F1D
// 首条位置: {"module":170,"inlinee":12061}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::render_config::impl$24::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":170,"record_offset":3304,"parent_record_offset":3276,"ancestor_record_offset":3276,"inlinee":10707,"source_line":75,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":170,"record_offset":3436,"parent_record_offset":3368,"ancestor_record_offset":3276,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":170,"record_offset":3468,"parent_record_offset":3436,"ancestor_record_offset":3276,"inlinee":10713,"source_line":29,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":170,"record_offset":3368,"parent_record_offset":3304,"ancestor_record_offset":3276,"inlinee":10709,"source_line":43,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":170,"record_offset":3408,"parent_record_offset":3368,"ancestor_record_offset":3276,"inlinee":10711,"source_line":13,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":170,"record_offset":3580,"parent_record_offset":3304,"ancestor_record_offset":3276,"inlinee":10594,"source_line":87,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::LastAppliedVideoSettings::needs_apply
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":170,"record_offset":1968,"rva":2951984,"va":5371661104}
// 编译实例 RVAs: 0x2d0b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31931963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661104,"raw_hex":"e83b3ee701","size":5,"soft_fail":false,"target_va":5403593344,"va":5371661376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31317469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661104,"raw_hex":"e8dddddd01","size":5,"soft_fail":false,"target_va":5402978880,"va":5371661406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":2220,"parent_record_offset":1968,"ancestor_record_offset":1968,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":170,"record_offset":2128,"parent_record_offset":1968,"ancestor_record_offset":1968,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":170,"record_offset":2156,"parent_record_offset":1968,"ancestor_record_offset":1968,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::PendingVideoSettingChanges::set
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":170,"record_offset":2192,"parent_record_offset":1968,"ancestor_record_offset":1968,"inlinee":12051,"source_line":627,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":2424,"parent_record_offset":2392,"ancestor_record_offset":1968,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":170,"record_offset":2452,"parent_record_offset":2424,"ancestor_record_offset":1968,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":170,"record_offset":2564,"parent_record_offset":2328,"ancestor_record_offset":1968,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":170,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":1968,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":170,"record_offset":2256,"parent_record_offset":2220,"ancestor_record_offset":1968,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":170,"record_offset":2292,"parent_record_offset":2256,"ancestor_record_offset":1968,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":170,"record_offset":2480,"parent_record_offset":2452,"ancestor_record_offset":1968,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":170,"record_offset":2508,"parent_record_offset":2480,"ancestor_record_offset":1968,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":170,"record_offset":2328,"parent_record_offset":2292,"ancestor_record_offset":1968,"inlinee":6196,"source_line":29,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":170,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":1968,"inlinee":12055,"source_line":5,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":170,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":1968,"inlinee":12053,"source_line":13,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::app_config::PendingVideoSettingChanges::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":170,"record_offset":2928,"parent_record_offset":2896,"ancestor_record_offset":1968,"inlinee":12057,"source_line":609,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::app_config::PendingVideoSettingChanges::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":170,"record_offset":2864,"parent_record_offset":2836,"ancestor_record_offset":1968,"inlinee":12056,"source_line":615,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":170,"record_offset":2964,"rva":2952304,"va":5371661424}
// 编译实例 RVAs: 0x2d0c70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2432370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e88ee2daff","size":5,"soft_fail":false,"target_va":5369230672,"va":5371663037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2049600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e8c0b9e0ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5371663115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2029312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e80009e1ff","size":5,"soft_fail":false,"target_va":5369632448,"va":5371661755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e1dd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2027662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e8720fe1ff","size":5,"soft_fail":false,"target_va":5369634256,"va":5371661913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e2ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2024435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e80d1ce1ff","size":5,"soft_fail":false,"target_va":5369637552,"va":5371661982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140339e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t428106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e84a880600","size":5,"soft_fail":false,"target_va":5372092032,"va":5371663921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033bd70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t436384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e8a0a80600","size":5,"soft_fail":false,"target_va":5372099952,"va":5371663563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033c8f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t439388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e85cb40600","size":5,"soft_fail":false,"target_va":5372102896,"va":5371663503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033cdf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t440320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e800b80600","size":5,"soft_fail":false,"target_va":5372104176,"va":5371663851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033d540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t442516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e894c00600","size":5,"soft_fail":false,"target_va":5372106048,"va":5371663527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033ded0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t444494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e84ec80600","size":5,"soft_fail":false,"target_va":5372108496,"va":5371663997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035dea0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t575336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e868c70800","size":5,"soft_fail":false,"target_va":5372239520,"va":5371664179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140361a60
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t590536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e8c8020900","size":5,"soft_fail":false,"target_va":5372254816,"va":5371664275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1086394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e8ba931000","size":5,"soft_fail":false,"target_va":5372749712,"va":5371663313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1486768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e8b0af1600","size":5,"soft_fail":false,"target_va":5373149840,"va":5371663067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043d460
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1490913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e8e1bf1600","size":5,"soft_fail":false,"target_va":5373154400,"va":5371663482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1496020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e8d4d31600","size":5,"soft_fail":false,"target_va":5373158800,"va":5371662775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5866898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e892855900","size":5,"soft_fail":false,"target_va":5377529888,"va":5371662985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-354040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e80899faff","size":5,"soft_fail":false,"target_va":5371308800,"va":5371662835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31314758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e846d3dd01","size":5,"soft_fail":false,"target_va":5402978880,"va":5371664117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2737595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e8bbc52900","size":5,"soft_fail":false,"target_va":5374399200,"va":5371661600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1496585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e809d61600","size":5,"soft_fail":false,"target_va":5373159392,"va":5371662802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::ImageView::blur_extend
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1483688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e8a8a31600","size":5,"soft_fail":false,"target_va":5373146752,"va":5371663059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::ImageView::tint
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1483632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371661424,"raw_hex":"e870a31600","size":5,"soft_fail":false,"target_va":5373146688,"va":5371663051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":170,"record_offset":4900,"parent_record_offset":2964,"ancestor_record_offset":2964,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":6560,"parent_record_offset":2964,"ancestor_record_offset":2964,"inlinee":12066,"source_line":1776,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":170,"record_offset":3144,"parent_record_offset":2964,"ancestor_record_offset":2964,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":170,"record_offset":3660,"parent_record_offset":2964,"ancestor_record_offset":2964,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":6076,"parent_record_offset":2964,"ancestor_record_offset":2964,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$14::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":170,"record_offset":3632,"parent_record_offset":2964,"ancestor_record_offset":2964,"inlinee":12063,"source_line":28,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::settings::LastAppliedVideoSettings::populate_empty_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":170,"record_offset":3204,"parent_record_offset":2964,"ancestor_record_offset":2964,"inlinee":12059,"source_line":62,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::fullscreen
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":170,"record_offset":4704,"parent_record_offset":2964,"ancestor_record_offset":2964,"inlinee":7513,"source_line":338,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":170,"record_offset":6336,"parent_record_offset":2964,"ancestor_record_offset":2964,"inlinee":7454,"source_line":349,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::with_layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":170,"record_offset":4984,"parent_record_offset":2964,"ancestor_record_offset":2964,"inlinee":5612,"source_line":833,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::impl$10::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":170,"record_offset":6532,"parent_record_offset":2964,"ancestor_record_offset":2964,"inlinee":12065,"source_line":152,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":6232,"parent_record_offset":6208,"ancestor_record_offset":2964,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":6208,"parent_record_offset":6152,"ancestor_record_offset":2964,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":170,"record_offset":4948,"parent_record_offset":4924,"ancestor_record_offset":2964,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":6180,"parent_record_offset":6152,"ancestor_record_offset":2964,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":4828,"parent_record_offset":4796,"ancestor_record_offset":2964,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":6152,"parent_record_offset":6124,"ancestor_record_offset":2964,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":4796,"parent_record_offset":4764,"ancestor_record_offset":2964,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":4764,"parent_record_offset":4728,"ancestor_record_offset":2964,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":170,"record_offset":3172,"parent_record_offset":3144,"ancestor_record_offset":2964,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":3712,"parent_record_offset":3684,"ancestor_record_offset":2964,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":3736,"parent_record_offset":3712,"ancestor_record_offset":2964,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":5800,"parent_record_offset":5776,"ancestor_record_offset":2964,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":170,"record_offset":5268,"parent_record_offset":5244,"ancestor_record_offset":2964,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":170,"record_offset":5352,"parent_record_offset":5328,"ancestor_record_offset":2964,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":170,"record_offset":5292,"parent_record_offset":5268,"ancestor_record_offset":2964,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":170,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":2964,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":170,"record_offset":3684,"parent_record_offset":3660,"ancestor_record_offset":2964,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":170,"record_offset":3240,"parent_record_offset":3204,"ancestor_record_offset":2964,"inlinee":5289,"source_line":1660,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":170,"record_offset":4924,"parent_record_offset":4900,"ancestor_record_offset":2964,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":170,"record_offset":6044,"parent_record_offset":4984,"ancestor_record_offset":2964,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":170,"record_offset":3436,"parent_record_offset":3368,"ancestor_record_offset":2964,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":6100,"parent_record_offset":6076,"ancestor_record_offset":2964,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":170,"record_offset":5712,"parent_record_offset":5688,"ancestor_record_offset":2964,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":170,"record_offset":5604,"parent_record_offset":5580,"ancestor_record_offset":2964,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":4860,"parent_record_offset":4764,"ancestor_record_offset":2964,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":170,"record_offset":3468,"parent_record_offset":3436,"ancestor_record_offset":2964,"inlinee":10713,"source_line":29,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":170,"record_offset":3368,"parent_record_offset":3304,"ancestor_record_offset":2964,"inlinee":10709,"source_line":43,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$24::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":170,"record_offset":3304,"parent_record_offset":3276,"ancestor_record_offset":2964,"inlinee":10707,"source_line":75,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":170,"record_offset":3408,"parent_record_offset":3368,"ancestor_record_offset":2964,"inlinee":10711,"source_line":13,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":170,"record_offset":5776,"parent_record_offset":5752,"ancestor_record_offset":2964,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":170,"record_offset":5064,"parent_record_offset":5040,"ancestor_record_offset":2964,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":170,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":2964,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":170,"record_offset":5752,"parent_record_offset":5532,"ancestor_record_offset":2964,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":170,"record_offset":5096,"parent_record_offset":5064,"ancestor_record_offset":2964,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":170,"record_offset":5664,"parent_record_offset":5640,"ancestor_record_offset":2964,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":170,"record_offset":5640,"parent_record_offset":5532,"ancestor_record_offset":2964,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":170,"record_offset":5580,"parent_record_offset":5556,"ancestor_record_offset":2964,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":170,"record_offset":5960,"parent_record_offset":5172,"ancestor_record_offset":2964,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":170,"record_offset":5556,"parent_record_offset":5532,"ancestor_record_offset":2964,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":170,"record_offset":5148,"parent_record_offset":5124,"ancestor_record_offset":2964,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":170,"record_offset":5124,"parent_record_offset":5064,"ancestor_record_offset":2964,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":170,"record_offset":5172,"parent_record_offset":5148,"ancestor_record_offset":2964,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":170,"record_offset":5412,"parent_record_offset":5384,"ancestor_record_offset":2964,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":170,"record_offset":5440,"parent_record_offset":5412,"ancestor_record_offset":2964,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":170,"record_offset":5500,"parent_record_offset":5384,"ancestor_record_offset":2964,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":170,"record_offset":5384,"parent_record_offset":5172,"ancestor_record_offset":2964,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":170,"record_offset":5216,"parent_record_offset":5172,"ancestor_record_offset":2964,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":170,"record_offset":5532,"parent_record_offset":5172,"ancestor_record_offset":2964,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":170,"record_offset":5244,"parent_record_offset":5172,"ancestor_record_offset":2964,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":170,"record_offset":5328,"parent_record_offset":5172,"ancestor_record_offset":2964,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":170,"record_offset":5868,"parent_record_offset":5172,"ancestor_record_offset":2964,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":170,"record_offset":3580,"parent_record_offset":3304,"ancestor_record_offset":2964,"inlinee":10594,"source_line":87,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":170,"record_offset":5016,"parent_record_offset":4984,"ancestor_record_offset":2964,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":170,"record_offset":6016,"parent_record_offset":4984,"ancestor_record_offset":2964,"inlinee":8019,"source_line":206,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::impl$1::populate_empty_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":170,"record_offset":3276,"parent_record_offset":3240,"ancestor_record_offset":2964,"inlinee":12061,"source_line":63,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":170,"record_offset":4728,"parent_record_offset":4704,"ancestor_record_offset":2964,"inlinee":7455,"source_line":398,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":170,"record_offset":5824,"parent_record_offset":5800,"ancestor_record_offset":2964,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F53
// 首条位置: {"module":58,"inlinee":8019}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3304
// 首条位置: {"module":220,"inlinee":13060}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":15208,"parent_record_offset":15136,"ancestor_record_offset":15136,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":220,"record_offset":15172,"parent_record_offset":15136,"ancestor_record_offset":15136,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":15360,"parent_record_offset":15336,"ancestor_record_offset":15136,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":15336,"parent_record_offset":15280,"ancestor_record_offset":15136,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":15308,"parent_record_offset":15280,"ancestor_record_offset":15136,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":15280,"parent_record_offset":15256,"ancestor_record_offset":15136,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":15232,"parent_record_offset":15208,"ancestor_record_offset":15136,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 216
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3306
// 首条位置: {"module":220,"inlinee":13062}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":19896,"parent_record_offset":19864,"ancestor_record_offset":19864,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":19924,"parent_record_offset":19896,"ancestor_record_offset":19864,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$3
// 记录形式: compiled_procedure; instances=1
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":220,"record_offset":42484,"rva":3410128,"va":5372119248}
// 编译实例 RVAs: 0x3408d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t630376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119248,"raw_hex":"e8689e0900","size":5,"soft_fail":false,"target_va":5372749696,"va":5372119315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t629756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119248,"raw_hex":"e8fc9b0900","size":5,"soft_fail":false,"target_va":5372749712,"va":5372119951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b1a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1025625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119248,"raw_hex":"e859a60f00","size":5,"soft_fail":false,"target_va":5373145504,"va":5372119874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1029944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119248,"raw_hex":"e838b70f00","size":5,"soft_fail":false,"target_va":5373149856,"va":5372119907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1059578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119248,"raw_hex":"e8fa2a1000","size":5,"soft_fail":false,"target_va":5373179424,"va":5372119841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056d1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2279155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119248,"raw_hex":"e8f3c62200","size":5,"soft_fail":false,"target_va":5374398944,"va":5372119784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30858072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119248,"raw_hex":"e858dbd601","size":5,"soft_fail":false,"target_va":5402978144,"va":5372120067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-812650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119248,"raw_hex":"e89699f3ff","size":5,"soft_fail":false,"target_va":5371306912,"va":5372119557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::_$LT$impl$u20$str$GT$::to_uppercase
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5413003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119248,"raw_hex":"e88b985200","size":5,"soft_fail":false,"target_va":5377532832,"va":5372119824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t4033530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119248,"raw_hex":"e8fa8b3d00","size":5,"soft_fail":false,"target_va":5376152880,"va":5372119345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1038944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119248,"raw_hex":"e860da0f00","size":5,"soft_fail":false,"target_va":5373158864,"va":5372119915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1026036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119248,"raw_hex":"e8f4a70f00","size":5,"soft_fail":false,"target_va":5373145904,"va":5372119863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::force_highlight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1026633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119248,"raw_hex":"e849aa0f00","size":5,"soft_fail":false,"target_va":5373146528,"va":5372119890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1025967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119248,"raw_hex":"e8afa70f00","size":5,"soft_fail":false,"target_va":5373145824,"va":5372119852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":42664,"parent_record_offset":42484,"ancestor_record_offset":42484,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":220,"record_offset":42812,"parent_record_offset":42484,"ancestor_record_offset":42484,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":42896,"parent_record_offset":42484,"ancestor_record_offset":42484,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":43196,"parent_record_offset":42484,"ancestor_record_offset":42484,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":220,"record_offset":43308,"parent_record_offset":42484,"ancestor_record_offset":42484,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":43252,"parent_record_offset":42484,"ancestor_record_offset":42484,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":220,"record_offset":43372,"parent_record_offset":42484,"ancestor_record_offset":42484,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":43428,"parent_record_offset":42484,"ancestor_record_offset":42484,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":42720,"parent_record_offset":42696,"ancestor_record_offset":42484,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":42744,"parent_record_offset":42720,"ancestor_record_offset":42484,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":43584,"parent_record_offset":43560,"ancestor_record_offset":42484,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":42696,"parent_record_offset":42664,"ancestor_record_offset":42484,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":43560,"parent_record_offset":43504,"ancestor_record_offset":42484,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":220,"record_offset":44868,"parent_record_offset":44844,"ancestor_record_offset":42484,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":43072,"parent_record_offset":43032,"ancestor_record_offset":42484,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":43532,"parent_record_offset":43504,"ancestor_record_offset":42484,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":42968,"parent_record_offset":42932,"ancestor_record_offset":42484,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":43504,"parent_record_offset":43476,"ancestor_record_offset":42484,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":220,"record_offset":42836,"parent_record_offset":42812,"ancestor_record_offset":42484,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":42932,"parent_record_offset":42896,"ancestor_record_offset":42484,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":42860,"parent_record_offset":42836,"ancestor_record_offset":42484,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":220,"record_offset":43860,"parent_record_offset":43836,"ancestor_record_offset":42484,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":220,"record_offset":43908,"parent_record_offset":43884,"ancestor_record_offset":42484,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":43276,"parent_record_offset":43252,"ancestor_record_offset":42484,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":220,"record_offset":44012,"parent_record_offset":43984,"ancestor_record_offset":42484,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":43452,"parent_record_offset":43428,"ancestor_record_offset":42484,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":220,"record_offset":43220,"parent_record_offset":43196,"ancestor_record_offset":42484,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":220,"record_offset":43340,"parent_record_offset":43308,"ancestor_record_offset":42484,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":42768,"parent_record_offset":42744,"ancestor_record_offset":42484,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":43000,"parent_record_offset":42896,"ancestor_record_offset":42484,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gui::impl$35::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":220,"record_offset":43396,"parent_record_offset":43372,"ancestor_record_offset":42484,"inlinee":13079,"source_line":227,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4
// 记录形式: compiled_procedure; instances=1
// 源码行: 256
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":220,"record_offset":44960,"rva":3411312,"va":5372120432}
// 编译实例 RVAs: 0x340d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14001de60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3297541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8fbaecdff","size":5,"soft_fail":false,"target_va":5368831584,"va":5372129120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14001e450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3292712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8d8c1cdff","size":5,"soft_fail":false,"target_va":5368833104,"va":5372125811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14001ea90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3288366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8d2d2cdff","size":5,"soft_fail":false,"target_va":5368834704,"va":5372123065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14001f020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3285744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e810ddcdff","size":5,"soft_fail":false,"target_va":5368836128,"va":5372121867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14001f5b0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3283472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8f0e5cdff","size":5,"soft_fail":false,"target_va":5368837552,"va":5372121019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14001fa50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3288648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8b8d1cdff","size":5,"soft_fail":false,"target_va":5368838736,"va":5372127379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140020030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3284683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e835e1cdff","size":5,"soft_fail":false,"target_va":5368840240,"va":5372124918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140020510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3284955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e825e0cdff","size":5,"soft_fail":false,"target_va":5368841488,"va":5372126438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140020ad0
//     owner=unresolved_namespace; count=14
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3279000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e868f7cdff","size":5,"soft_fail":false,"target_va":5368842960,"va":5372121955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140020f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3283638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e84ae5cdff","size":5,"soft_fail":false,"target_va":5368844160,"va":5372127793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400515f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3082765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8f3f5d0ff","size":5,"soft_fail":false,"target_va":5369042416,"va":5372125176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2893300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e80cdad3ff","size":5,"soft_fail":false,"target_va":5369228752,"va":5372122047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ef50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2891490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e81ee1d3ff","size":5,"soft_fail":false,"target_va":5369229136,"va":5372120621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f4e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2893944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e888d7d3ff","size":5,"soft_fail":false,"target_va":5369230560,"va":5372124499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2506379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e875c1d9ff","size":5,"soft_fail":false,"target_va":5369616080,"va":5372122454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2505996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8f4c2d9ff","size":5,"soft_fail":false,"target_va":5369616480,"va":5372122471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e0680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2499229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e863ddd9ff","size":5,"soft_fail":false,"target_va":5369628288,"va":5372127512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e4fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2480431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8d126daff","size":5,"soft_fail":false,"target_va":5369647056,"va":5372127482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e5460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2477932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e89430daff","size":5,"soft_fail":false,"target_va":5369648224,"va":5372126151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e5880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2475517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8033adaff","size":5,"soft_fail":false,"target_va":5369649280,"va":5372124792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e6460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2473602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e87e41daff","size":5,"soft_fail":false,"target_va":5369652320,"va":5372125917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2309037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e853c4dcff","size":5,"soft_fail":false,"target_va":5369813552,"va":5372122584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-821794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8de75f3ff","size":5,"soft_fail":false,"target_va":5371298848,"va":5372120637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f8b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-300976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e85068fbff","size":5,"soft_fail":false,"target_va":5371824976,"va":5372125947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-299956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e84c6cfbff","size":5,"soft_fail":false,"target_va":5371827552,"va":5372127503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037c010
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t236183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8979a0300","size":5,"soft_fail":false,"target_va":5372362768,"va":5372126580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140382c20
//     owner=unresolved_namespace; count=2
//     target_aliases=37 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t264770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8420a0400","size":5,"soft_fail":false,"target_va":5372390432,"va":5372125657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t623495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e887830900","size":5,"soft_fail":false,"target_va":5372749696,"va":5372126196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=19
//     target_aliases=1 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t627945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8e9940900","size":5,"soft_fail":false,"target_va":5372749712,"va":5372121762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=3
//     target_aliases=53 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1025646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e86ea60f00","size":5,"soft_fail":false,"target_va":5373149840,"va":5372124189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=4
//     target_aliases=8 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1029205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e855b40f00","size":5,"soft_fail":false,"target_va":5373149856,"va":5372120646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043d460
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1031882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8cabe0f00","size":5,"soft_fail":false,"target_va":5373154400,"va":5372122513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1036385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e861d00f00","size":5,"soft_fail":false,"target_va":5373158800,"va":5372122410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043f210
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1039442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e852dc0f00","size":5,"soft_fail":false,"target_va":5373162000,"va":5372122553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443570
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1058670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e86e271000","size":5,"soft_fail":false,"target_va":5373179248,"va":5372120573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1058815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8ff271000","size":5,"soft_fail":false,"target_va":5373179424,"va":5372120604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404442d0
//     owner=unresolved_namespace; count=30
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1061750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e876331000","size":5,"soft_fail":false,"target_va":5373182672,"va":5372120917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140444300
//     owner=unresolved_namespace; count=30
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1061793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8a1331000","size":5,"soft_fail":false,"target_va":5373182720,"va":5372120922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140549fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2129950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e81e802000","size":5,"soft_fail":false,"target_va":5374255072,"va":5372125117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14065ad20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3247552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8c08d3100","size":5,"soft_fail":false,"target_va":5375372576,"va":5372125019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31467027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e81326e001","size":5,"soft_fail":false,"target_va":5403593344,"va":5372126312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30848149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e895b4d601","size":5,"soft_fail":false,"target_va":5402978144,"va":5372129990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5408295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e827865200","size":5,"soft_fail":false,"target_va":5377529888,"va":5372121588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-815647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8e18df3ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5372120698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::_$LT$impl$u20$str$GT$::to_uppercase
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5410823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e807905200","size":5,"soft_fail":false,"target_va":5377532832,"va":5372122004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::EventReader$LT$E$GT$::read
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-1464757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e84ba6e9ff","size":5,"soft_fail":false,"target_va":5370659792,"va":5372124544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t30851928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e858c3d601","size":5,"soft_fail":false,"target_va":5402979344,"va":5372127411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::iter::impl$181::find<country_core::resources::window::AvailableMonitor,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$4>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e815bcffff","size":5,"soft_fail":false,"target_va":5372109120,"va":5372126502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::render_config::FullscreenConfig::resolution
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1903489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8810b1d00","size":5,"soft_fail":false,"target_va":5374030608,"va":5372127114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::render_config::impl$11::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e89cbbffff","size":5,"soft_fail":false,"target_va":5372109024,"va":5372126527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=42
//     evidence={"assembly":"\tcallq\t4032323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e843873d00","size":5,"soft_fail":false,"target_va":5376152880,"va":5372120552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5229451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e88bcb4f00","size":5,"soft_fail":false,"target_va":5377353776,"va":5372124320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::device_uuid
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4190036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e854ef3f00","size":5,"soft_fail":false,"target_va":5376315072,"va":5372125031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::gui::gui_controls::show_gui_controls
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-336817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e84fdcfaff","size":5,"soft_fail":false,"target_va":5371786720,"va":5372123532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::gui::language_menu::do_gui_language_menu
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2981496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e88881d2ff","size":5,"soft_fail":false,"target_va":5369142960,"va":5372124451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::gui::settings::LastAppliedVideoSettings::needs_apply
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-468580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e89cd9f8ff","size":5,"soft_fail":false,"target_va":5371661104,"va":5372129679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::vram::calculate_max_resolution_scale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-596741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8fbe4f6ff","size":5,"soft_fail":false,"target_va":5371531440,"va":5372128176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-595889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e84fe8f6ff","size":5,"soft_fail":false,"target_va":5371532240,"va":5372128124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$17,ref$<str$> >
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8fea3ffff","size":5,"soft_fail":false,"target_va":5372101888,"va":5372125437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$18,ref$<str$> >
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e827baffff","size":5,"soft_fail":false,"target_va":5372107600,"va":5372125476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$19,ref$<str$> >
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e88a96ffff","size":5,"soft_fail":false,"target_va":5372098544,"va":5372125537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$20,ref$<str$> >
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-29847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8698bffff","size":5,"soft_fail":false,"target_va":5372095728,"va":5372125570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$21,ref$<str$> >
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-29903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8318bffff","size":5,"soft_fail":false,"target_va":5372094816,"va":5372124714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$22,ref$<str$> >
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8da97ffff","size":5,"soft_fail":false,"target_va":5372098096,"va":5372124753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t1038205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e87dd70f00","size":5,"soft_fail":false,"target_va":5373158864,"va":5372120654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1038377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e829d80f00","size":5,"soft_fail":false,"target_va":5373160864,"va":5372122482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::has_global_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1038844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e8fcd90f00","size":5,"soft_fail":false,"target_va":5373161488,"va":5372122639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1022054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e866980f00","size":5,"soft_fail":false,"target_va":5373146240,"va":5372124181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::weight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1024090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120432,"raw_hex":"e85aa00f00","size":5,"soft_fail":false,"target_va":5373146176,"va":5372122081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":220,"record_offset":45784,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":220,"record_offset":49392,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":8700,"source_line":207,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":51736,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":12918,"source_line":2673,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":52504,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":52888,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=31
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":45500,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":220,"record_offset":45244,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send_default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":220,"record_offset":55972,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":7452,"source_line":641,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":220,"record_offset":48292,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":220,"record_offset":48516,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":220,"record_offset":54564,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":220,"record_offset":48656,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":47284,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":47256,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":52832,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":45924,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":220,"record_offset":48572,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":220,"record_offset":52140,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":9336,"source_line":976,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::find
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":220,"record_offset":50088,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":8711,"source_line":296,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::position
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":220,"record_offset":49448,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":8990,"source_line":331,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::PendingVideoSettingChanges::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":220,"record_offset":55680,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":10700,"source_line":585,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::settings::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":49224,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":12081,"source_line":1026,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::TemplateView::with_slot
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":220,"record_offset":46676,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":13081,"source_line":226,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":220,"record_offset":46560,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":220,"record_offset":46236,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":220,"record_offset":45212,"parent_record_offset":44960,"ancestor_record_offset":44960,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":51084,"parent_record_offset":51060,"ancestor_record_offset":44960,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":51108,"parent_record_offset":51084,"ancestor_record_offset":44960,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":46076,"parent_record_offset":46052,"ancestor_record_offset":44960,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":51060,"parent_record_offset":50960,"ancestor_record_offset":44960,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":46052,"parent_record_offset":45996,"ancestor_record_offset":44960,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":220,"record_offset":45832,"parent_record_offset":45808,"ancestor_record_offset":44960,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":46024,"parent_record_offset":45996,"ancestor_record_offset":44960,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":45428,"parent_record_offset":45404,"ancestor_record_offset":44960,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":50960,"parent_record_offset":50924,"ancestor_record_offset":44960,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":50924,"parent_record_offset":50888,"ancestor_record_offset":44960,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":51180,"parent_record_offset":50924,"ancestor_record_offset":44960,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":45996,"parent_record_offset":45972,"ancestor_record_offset":44960,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":220,"record_offset":49416,"parent_record_offset":49392,"ancestor_record_offset":44960,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":220,"record_offset":49712,"parent_record_offset":49684,"ancestor_record_offset":44960,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":45404,"parent_record_offset":45364,"ancestor_record_offset":44960,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":51768,"parent_record_offset":51736,"ancestor_record_offset":44960,"inlinee":8654,"source_line":1815,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":51248,"parent_record_offset":51220,"ancestor_record_offset":44960,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":45364,"parent_record_offset":45328,"ancestor_record_offset":44960,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":50852,"parent_record_offset":50808,"ancestor_record_offset":44960,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":50888,"parent_record_offset":50852,"ancestor_record_offset":44960,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":48680,"parent_record_offset":48656,"ancestor_record_offset":44960,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":51456,"parent_record_offset":51416,"ancestor_record_offset":44960,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$5::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":51824,"parent_record_offset":51768,"ancestor_record_offset":44960,"inlinee":8528,"source_line":2674,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":220,"record_offset":49744,"parent_record_offset":49712,"ancestor_record_offset":44960,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":220,"record_offset":51548,"parent_record_offset":51456,"ancestor_record_offset":44960,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":220,"record_offset":51220,"parent_record_offset":50808,"ancestor_record_offset":44960,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":220,"record_offset":50768,"parent_record_offset":50728,"ancestor_record_offset":44960,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":220,"record_offset":50808,"parent_record_offset":50768,"ancestor_record_offset":44960,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=35
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":45296,"parent_record_offset":45268,"ancestor_record_offset":44960,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":45268,"parent_record_offset":45244,"ancestor_record_offset":44960,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":220,"record_offset":45328,"parent_record_offset":45244,"ancestor_record_offset":44960,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":220,"record_offset":48316,"parent_record_offset":48292,"ancestor_record_offset":44960,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$28::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/keyboard.rs
//     evidence={"module":220,"record_offset":49128,"parent_record_offset":49104,"ancestor_record_offset":44960,"inlinee":12072,"source_line":200,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::mouse::impl$11::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/mouse.rs
//     evidence={"module":220,"record_offset":48540,"parent_record_offset":48516,"ancestor_record_offset":44960,"inlinee":13089,"source_line":50,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":220,"record_offset":51004,"parent_record_offset":50960,"ancestor_record_offset":44960,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":220,"record_offset":51028,"parent_record_offset":51004,"ancestor_record_offset":44960,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":47508,"parent_record_offset":47484,"ancestor_record_offset":44960,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":47532,"parent_record_offset":47508,"ancestor_record_offset":44960,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":220,"record_offset":46904,"parent_record_offset":46676,"ancestor_record_offset":44960,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":220,"record_offset":51848,"parent_record_offset":51824,"ancestor_record_offset":44960,"inlinee":8530,"source_line":1709,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":220,"record_offset":49684,"parent_record_offset":49656,"ancestor_record_offset":44960,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":220,"record_offset":46932,"parent_record_offset":46676,"ancestor_record_offset":44960,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":220,"record_offset":52104,"parent_record_offset":51768,"ancestor_record_offset":44960,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":220,"record_offset":48348,"parent_record_offset":48316,"ancestor_record_offset":44960,"inlinee":8099,"source_line":143,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":220,"record_offset":51304,"parent_record_offset":51276,"ancestor_record_offset":44960,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":220,"record_offset":51376,"parent_record_offset":51332,"ancestor_record_offset":44960,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":220,"record_offset":50652,"parent_record_offset":50628,"ancestor_record_offset":44960,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":220,"record_offset":51276,"parent_record_offset":51248,"ancestor_record_offset":44960,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":220,"record_offset":51416,"parent_record_offset":51376,"ancestor_record_offset":44960,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":220,"record_offset":52288,"parent_record_offset":52240,"ancestor_record_offset":44960,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":220,"record_offset":52316,"parent_record_offset":52288,"ancestor_record_offset":44960,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":220,"record_offset":49984,"parent_record_offset":49956,"ancestor_record_offset":44960,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":48384,"parent_record_offset":48348,"ancestor_record_offset":44960,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":50564,"parent_record_offset":50288,"ancestor_record_offset":44960,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":45808,"parent_record_offset":45784,"ancestor_record_offset":44960,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":48448,"parent_record_offset":48348,"ancestor_record_offset":44960,"inlinee":8100,"source_line":1485,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":48804,"parent_record_offset":48772,"ancestor_record_offset":44960,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":48836,"parent_record_offset":48804,"ancestor_record_offset":44960,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":48480,"parent_record_offset":48316,"ancestor_record_offset":44960,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":45948,"parent_record_offset":45924,"ancestor_record_offset":44960,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":220,"record_offset":48620,"parent_record_offset":48596,"ancestor_record_offset":44960,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":220,"record_offset":50252,"parent_record_offset":50228,"ancestor_record_offset":44960,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":220,"record_offset":47424,"parent_record_offset":47380,"ancestor_record_offset":44960,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":52360,"parent_record_offset":52316,"ancestor_record_offset":44960,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":51136,"parent_record_offset":51108,"ancestor_record_offset":44960,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":45460,"parent_record_offset":45364,"ancestor_record_offset":44960,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":220,"record_offset":49776,"parent_record_offset":49744,"ancestor_record_offset":44960,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":220,"record_offset":49808,"parent_record_offset":49776,"ancestor_record_offset":44960,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::as_mut_ptr_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":220,"record_offset":52176,"parent_record_offset":52140,"ancestor_record_offset":44960,"inlinee":13092,"source_line":873,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":220,"record_offset":52240,"parent_record_offset":52140,"ancestor_record_offset":44960,"inlinee":9338,"source_line":999,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":220,"record_offset":52916,"parent_record_offset":52888,"ancestor_record_offset":44960,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":220,"record_offset":52944,"parent_record_offset":52916,"ancestor_record_offset":44960,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":220,"record_offset":48596,"parent_record_offset":48572,"ancestor_record_offset":44960,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":220,"record_offset":50228,"parent_record_offset":50132,"ancestor_record_offset":44960,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":220,"record_offset":51332,"parent_record_offset":51304,"ancestor_record_offset":44960,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":220,"record_offset":47380,"parent_record_offset":47340,"ancestor_record_offset":44960,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::position
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":220,"record_offset":47340,"parent_record_offset":47312,"ancestor_record_offset":44960,"inlinee":8990,"source_line":331,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":220,"record_offset":46844,"parent_record_offset":46820,"ancestor_record_offset":44960,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":220,"record_offset":46868,"parent_record_offset":46844,"ancestor_record_offset":44960,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":220,"record_offset":46960,"parent_record_offset":46676,"ancestor_record_offset":44960,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":220,"record_offset":46820,"parent_record_offset":46676,"ancestor_record_offset":44960,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":47312,"parent_record_offset":47284,"ancestor_record_offset":44960,"inlinee":13082,"source_line":371,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":47460,"parent_record_offset":47340,"ancestor_record_offset":44960,"inlinee":13084,"source_line":375,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$10
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":53076,"parent_record_offset":52984,"ancestor_record_offset":44960,"inlinee":13097,"source_line":539,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$12
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":53448,"parent_record_offset":53332,"ancestor_record_offset":44960,"inlinee":13098,"source_line":568,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$13
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":53504,"parent_record_offset":53480,"ancestor_record_offset":44960,"inlinee":13099,"source_line":569,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$14
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":53728,"parent_record_offset":53632,"ancestor_record_offset":44960,"inlinee":13100,"source_line":586,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$16
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":47104,"parent_record_offset":46676,"ancestor_record_offset":44960,"inlinee":13087,"source_line":745,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":49568,"parent_record_offset":49448,"ancestor_record_offset":44960,"inlinee":13090,"source_line":462,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$5
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":50288,"parent_record_offset":50088,"ancestor_record_offset":44960,"inlinee":13091,"source_line":492,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$8
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":52560,"parent_record_offset":52532,"ancestor_record_offset":44960,"inlinee":13093,"source_line":519,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$8::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":52612,"parent_record_offset":52584,"ancestor_record_offset":44960,"inlinee":13095,"source_line":519,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$9
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":52856,"parent_record_offset":52832,"ancestor_record_offset":44960,"inlinee":13096,"source_line":520,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::impl$17::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":51872,"parent_record_offset":51848,"ancestor_record_offset":44960,"inlinee":8532,"source_line":992,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":220,"record_offset":46988,"parent_record_offset":46676,"ancestor_record_offset":44960,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":220,"record_offset":46608,"parent_record_offset":46584,"ancestor_record_offset":44960,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":220,"record_offset":46640,"parent_record_offset":46584,"ancestor_record_offset":44960,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":220,"record_offset":46584,"parent_record_offset":46560,"ancestor_record_offset":44960,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$19::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.1.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/uuid-1.1.2/src/lib.rs
//     evidence={"module":220,"record_offset":47484,"parent_record_offset":47460,"ancestor_record_offset":44960,"inlinee":13086,"source_line":397,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 371
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x331A
// 首条位置: {"module":220,"inlinee":13082}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$181::position
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":220,"record_offset":47340,"parent_record_offset":47312,"ancestor_record_offset":47312,"inlinee":8990,"source_line":331,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":47508,"parent_record_offset":47484,"ancestor_record_offset":47312,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":47532,"parent_record_offset":47508,"ancestor_record_offset":47312,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":220,"record_offset":47424,"parent_record_offset":47380,"ancestor_record_offset":47312,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":220,"record_offset":47380,"parent_record_offset":47340,"ancestor_record_offset":47312,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":47460,"parent_record_offset":47340,"ancestor_record_offset":47312,"inlinee":13084,"source_line":375,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$19::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.1.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/uuid-1.1.2/src/lib.rs
//     evidence={"module":220,"record_offset":47484,"parent_record_offset":47460,"ancestor_record_offset":47312,"inlinee":13086,"source_line":397,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 375
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x331C
// 首条位置: {"module":220,"inlinee":13084}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] uuid::impl$19::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.1.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/uuid-1.1.2/src/lib.rs
//     evidence={"module":220,"record_offset":47484,"parent_record_offset":47460,"ancestor_record_offset":47460,"inlinee":13086,"source_line":397,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":47508,"parent_record_offset":47484,"ancestor_record_offset":47460,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":47532,"parent_record_offset":47508,"ancestor_record_offset":47460,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 381
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17DB
// 首条位置: {"module":21,"inlinee":6107}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":21,"record_offset":17504,"parent_record_offset":16756,"ancestor_record_offset":16756,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":21,"record_offset":16804,"parent_record_offset":16756,"ancestor_record_offset":16756,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::trim
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":21,"record_offset":17476,"parent_record_offset":16756,"ancestor_record_offset":16756,"inlinee":6115,"source_line":1946,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":21,"record_offset":17324,"parent_record_offset":16756,"ancestor_record_offset":16756,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":17168,"parent_record_offset":17144,"ancestor_record_offset":16756,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":17192,"parent_record_offset":17168,"ancestor_record_offset":16756,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":17144,"parent_record_offset":17112,"ancestor_record_offset":16756,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":21,"record_offset":17552,"parent_record_offset":17528,"ancestor_record_offset":16756,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":17112,"parent_record_offset":17084,"ancestor_record_offset":16756,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":17084,"parent_record_offset":17056,"ancestor_record_offset":16756,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":17264,"parent_record_offset":17084,"ancestor_record_offset":16756,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":21,"record_offset":16964,"parent_record_offset":16932,"ancestor_record_offset":16756,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":21,"record_offset":16932,"parent_record_offset":16900,"ancestor_record_offset":16756,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":21,"record_offset":16868,"parent_record_offset":16836,"ancestor_record_offset":16756,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":21,"record_offset":16900,"parent_record_offset":16868,"ancestor_record_offset":16756,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":21,"record_offset":16836,"parent_record_offset":16804,"ancestor_record_offset":16756,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":21,"record_offset":17056,"parent_record_offset":16964,"ancestor_record_offset":16756,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":21,"record_offset":17024,"parent_record_offset":17000,"ancestor_record_offset":16756,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":21,"record_offset":17432,"parent_record_offset":17408,"ancestor_record_offset":16756,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":21,"record_offset":17528,"parent_record_offset":17504,"ancestor_record_offset":16756,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":21,"record_offset":17000,"parent_record_offset":16964,"ancestor_record_offset":16756,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":17220,"parent_record_offset":17192,"ancestor_record_offset":16756,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":21,"record_offset":17408,"parent_record_offset":17384,"ancestor_record_offset":16756,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":21,"record_offset":17384,"parent_record_offset":17352,"ancestor_record_offset":16756,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":21,"record_offset":17352,"parent_record_offset":17324,"ancestor_record_offset":16756,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 456
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x21A8
// 首条位置: {"module":73,"inlinee":8616}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":73,"record_offset":91872,"parent_record_offset":91852,"ancestor_record_offset":91852,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 462
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3322
// 首条位置: {"module":220,"inlinee":13090}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":49600,"parent_record_offset":49568,"ancestor_record_offset":49568,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":49628,"parent_record_offset":49568,"ancestor_record_offset":49568,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":220,"record_offset":49712,"parent_record_offset":49684,"ancestor_record_offset":49568,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":220,"record_offset":49744,"parent_record_offset":49712,"ancestor_record_offset":49568,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":220,"record_offset":49684,"parent_record_offset":49656,"ancestor_record_offset":49568,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":49656,"parent_record_offset":49628,"ancestor_record_offset":49568,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":220,"record_offset":49776,"parent_record_offset":49744,"ancestor_record_offset":49568,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":220,"record_offset":49808,"parent_record_offset":49776,"ancestor_record_offset":49568,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 480
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x330A
// 首条位置: {"module":220,"inlinee":13066}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":23740,"parent_record_offset":23712,"ancestor_record_offset":23712,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":24060,"parent_record_offset":23712,"ancestor_record_offset":23712,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":23836,"parent_record_offset":23712,"ancestor_record_offset":23712,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":220,"record_offset":23924,"parent_record_offset":23900,"ancestor_record_offset":23712,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":220,"record_offset":23952,"parent_record_offset":23924,"ancestor_record_offset":23712,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":220,"record_offset":23900,"parent_record_offset":23868,"ancestor_record_offset":23712,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":23868,"parent_record_offset":23836,"ancestor_record_offset":23712,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":220,"record_offset":23976,"parent_record_offset":23952,"ancestor_record_offset":23712,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":220,"record_offset":24004,"parent_record_offset":23976,"ancestor_record_offset":23712,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":220,"record_offset":23768,"parent_record_offset":23740,"ancestor_record_offset":23712,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":220,"record_offset":23796,"parent_record_offset":23768,"ancestor_record_offset":23712,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 492
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3323
// 首条位置: {"module":220,"inlinee":13091}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":50564,"parent_record_offset":50288,"ancestor_record_offset":50288,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":50320,"parent_record_offset":50288,"ancestor_record_offset":50288,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":220,"record_offset":50420,"parent_record_offset":50396,"ancestor_record_offset":50288,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":220,"record_offset":50448,"parent_record_offset":50420,"ancestor_record_offset":50288,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":220,"record_offset":50396,"parent_record_offset":50356,"ancestor_record_offset":50288,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":50356,"parent_record_offset":50320,"ancestor_record_offset":50288,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":220,"record_offset":50476,"parent_record_offset":50448,"ancestor_record_offset":50288,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":220,"record_offset":50504,"parent_record_offset":50476,"ancestor_record_offset":50288,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 497
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x219E
// 首条位置: {"module":73,"inlinee":8606}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":73,"record_offset":75396,"parent_record_offset":75376,"ancestor_record_offset":75376,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":73,"record_offset":75444,"parent_record_offset":75420,"ancestor_record_offset":75376,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":73,"record_offset":75420,"parent_record_offset":75396,"ancestor_record_offset":75376,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 519
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3325
// 首条位置: {"module":220,"inlinee":13093}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$181::position
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":220,"record_offset":52584,"parent_record_offset":52560,"ancestor_record_offset":52560,"inlinee":8990,"source_line":331,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":52636,"parent_record_offset":52612,"ancestor_record_offset":52560,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":52660,"parent_record_offset":52636,"ancestor_record_offset":52560,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":220,"record_offset":52756,"parent_record_offset":52732,"ancestor_record_offset":52560,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":220,"record_offset":52788,"parent_record_offset":52696,"ancestor_record_offset":52560,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":220,"record_offset":52732,"parent_record_offset":52696,"ancestor_record_offset":52560,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":220,"record_offset":52696,"parent_record_offset":52584,"ancestor_record_offset":52560,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$8::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":52612,"parent_record_offset":52584,"ancestor_record_offset":52560,"inlinee":13095,"source_line":519,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$8::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 519
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3327
// 首条位置: {"module":220,"inlinee":13095}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":52636,"parent_record_offset":52612,"ancestor_record_offset":52612,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":52660,"parent_record_offset":52636,"ancestor_record_offset":52612,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$9
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 520
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3328
// 首条位置: {"module":220,"inlinee":13096}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$10
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 539
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3329
// 首条位置: {"module":220,"inlinee":13097}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":53100,"parent_record_offset":53076,"ancestor_record_offset":53076,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":53124,"parent_record_offset":53100,"ancestor_record_offset":53076,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$11
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 551
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2199
// 首条位置: {"module":73,"inlinee":8601}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":73,"record_offset":70472,"parent_record_offset":70448,"ancestor_record_offset":70448,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":73,"record_offset":70496,"parent_record_offset":70472,"ancestor_record_offset":70448,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$12
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 568
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x332A
// 首条位置: {"module":220,"inlinee":13098}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$13
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 569
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x332B
// 首条位置: {"module":220,"inlinee":13099}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$14
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 586
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x332C
// 首条位置: {"module":220,"inlinee":13100}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":53756,"parent_record_offset":53728,"ancestor_record_offset":53728,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":53780,"parent_record_offset":53756,"ancestor_record_offset":53728,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$16
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 745
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x331F
// 首条位置: {"module":220,"inlinee":13087}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":47128,"parent_record_offset":47104,"ancestor_record_offset":47104,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":47156,"parent_record_offset":47128,"ancestor_record_offset":47104,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$17
// 记录形式: compiled_procedure; instances=1
// 源码行: 818
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":220,"record_offset":58352,"rva":3421888,"va":5372131008}
// 编译实例 RVAs: 0x3436c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2902405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372131008,"raw_hex":"e87bb6d3ff","size":5,"soft_fail":false,"target_va":5369228752,"va":5372131152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2900430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372131008,"raw_hex":"e832bed3ff","size":5,"soft_fail":false,"target_va":5369230672,"va":5372131097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-832325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372131008,"raw_hex":"e8bb4cf3ff","size":5,"soft_fail":false,"target_va":5371298848,"va":5372131168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t618196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372131008,"raw_hex":"e8d46e0900","size":5,"soft_fail":false,"target_va":5372749696,"va":5372131495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t618131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372131008,"raw_hex":"e8936e0900","size":5,"soft_fail":false,"target_va":5372749712,"va":5372131576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443570
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1048108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372131008,"raw_hex":"e82cfe0f00","size":5,"soft_fail":false,"target_va":5373179248,"va":5372131135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31461815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372131008,"raw_hex":"e8b711e001","size":5,"soft_fail":false,"target_va":5403593360,"va":5372131540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30846277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372131008,"raw_hex":"e845add601","size":5,"soft_fail":false,"target_va":5402978144,"va":5372131862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5398482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372131008,"raw_hex":"e8d25f5200","size":5,"soft_fail":false,"target_va":5377529888,"va":5372131401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5395268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372131008,"raw_hex":"e844535200","size":5,"soft_fail":false,"target_va":5377527120,"va":5372131847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30847035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372131008,"raw_hex":"e83bb0d601","size":5,"soft_fail":false,"target_va":5402978880,"va":5372131840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_key_bindings::KeyBinding::action_desc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1953447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372131008,"raw_hex":"e8a7ce1d00","size":5,"soft_fail":false,"target_va":5374084512,"va":5372131060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_key_bindings::KeyBinding::get_keycode
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1954847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372131008,"raw_hex":"e81fd41d00","size":5,"soft_fail":false,"target_va":5374086064,"va":5372131212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_key_bindings::keycode_to_str
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1955089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372131008,"raw_hex":"e811d51d00","size":5,"soft_fail":false,"target_va":5374086352,"va":5372131258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4021555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372131008,"raw_hex":"e8335d3d00","size":5,"soft_fail":false,"target_va":5376152880,"va":5372131320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":220,"record_offset":58720,"parent_record_offset":58352,"ancestor_record_offset":58352,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":58536,"parent_record_offset":58352,"ancestor_record_offset":58352,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":58632,"parent_record_offset":58352,"ancestor_record_offset":58352,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":58564,"parent_record_offset":58352,"ancestor_record_offset":58352,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":60036,"parent_record_offset":58352,"ancestor_record_offset":58352,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":220,"record_offset":58600,"parent_record_offset":58352,"ancestor_record_offset":58352,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":59284,"parent_record_offset":59260,"ancestor_record_offset":58352,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":59308,"parent_record_offset":59284,"ancestor_record_offset":58352,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":59664,"parent_record_offset":59640,"ancestor_record_offset":58352,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":59260,"parent_record_offset":59224,"ancestor_record_offset":58352,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":59640,"parent_record_offset":59584,"ancestor_record_offset":58352,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":220,"record_offset":58768,"parent_record_offset":58744,"ancestor_record_offset":58352,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":59612,"parent_record_offset":59584,"ancestor_record_offset":58352,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":59224,"parent_record_offset":59196,"ancestor_record_offset":58352,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":59196,"parent_record_offset":59168,"ancestor_record_offset":58352,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":59380,"parent_record_offset":59196,"ancestor_record_offset":58352,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":59584,"parent_record_offset":59556,"ancestor_record_offset":58352,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":220,"record_offset":59132,"parent_record_offset":59100,"ancestor_record_offset":58352,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":220,"record_offset":59100,"parent_record_offset":59068,"ancestor_record_offset":58352,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":220,"record_offset":59036,"parent_record_offset":58988,"ancestor_record_offset":58352,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":220,"record_offset":59068,"parent_record_offset":59036,"ancestor_record_offset":58352,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":220,"record_offset":58988,"parent_record_offset":58956,"ancestor_record_offset":58352,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":220,"record_offset":58956,"parent_record_offset":58924,"ancestor_record_offset":58352,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":220,"record_offset":58924,"parent_record_offset":58852,"ancestor_record_offset":58352,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":59168,"parent_record_offset":59132,"ancestor_record_offset":58352,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":220,"record_offset":59444,"parent_record_offset":59420,"ancestor_record_offset":58352,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":58744,"parent_record_offset":58720,"ancestor_record_offset":58352,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":58684,"parent_record_offset":58660,"ancestor_record_offset":58352,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":220,"record_offset":59420,"parent_record_offset":59132,"ancestor_record_offset":58352,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":59508,"parent_record_offset":58804,"ancestor_record_offset":58352,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":59336,"parent_record_offset":59308,"ancestor_record_offset":58352,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$17::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":58660,"parent_record_offset":58632,"ancestor_record_offset":58352,"inlinee":13102,"source_line":830,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$17::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":220,"record_offset":58852,"parent_record_offset":58804,"ancestor_record_offset":58352,"inlinee":13103,"source_line":836,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$17::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 830
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x332E
// 首条位置: {"module":220,"inlinee":13102}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":58684,"parent_record_offset":58660,"ancestor_record_offset":58660,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$17::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 836
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x332F
// 首条位置: {"module":220,"inlinee":13103}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":220,"record_offset":58924,"parent_record_offset":58852,"ancestor_record_offset":58852,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":58892,"parent_record_offset":58852,"ancestor_record_offset":58852,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":59284,"parent_record_offset":59260,"ancestor_record_offset":58852,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":59308,"parent_record_offset":59284,"ancestor_record_offset":58852,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":59260,"parent_record_offset":59224,"ancestor_record_offset":58852,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":59224,"parent_record_offset":59196,"ancestor_record_offset":58852,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":59196,"parent_record_offset":59168,"ancestor_record_offset":58852,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":59380,"parent_record_offset":59196,"ancestor_record_offset":58852,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":220,"record_offset":59132,"parent_record_offset":59100,"ancestor_record_offset":58852,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":220,"record_offset":59100,"parent_record_offset":59068,"ancestor_record_offset":58852,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":220,"record_offset":59036,"parent_record_offset":58988,"ancestor_record_offset":58852,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":220,"record_offset":59068,"parent_record_offset":59036,"ancestor_record_offset":58852,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":220,"record_offset":58988,"parent_record_offset":58956,"ancestor_record_offset":58852,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":220,"record_offset":58956,"parent_record_offset":58924,"ancestor_record_offset":58852,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":59168,"parent_record_offset":59132,"ancestor_record_offset":58852,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":220,"record_offset":59444,"parent_record_offset":59420,"ancestor_record_offset":58852,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":220,"record_offset":59420,"parent_record_offset":59132,"ancestor_record_offset":58852,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":59336,"parent_record_offset":59308,"ancestor_record_offset":58852,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$18
// 记录形式: compiled_procedure; instances=1
// 源码行: 846
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":220,"record_offset":61304,"rva":3423072,"va":5372132192}
// 编译实例 RVAs: 0x343b60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=2
//     target_aliases=8 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1017558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132192,"raw_hex":"e8d6860f00","size":5,"soft_fail":false,"target_va":5373149856,"va":5372132293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1047146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132192,"raw_hex":"e86afa0f00","size":5,"soft_fail":false,"target_va":5373179424,"va":5372132273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-827300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132192,"raw_hex":"e85c60f3ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5372132351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_key_bindings::KeyBindings::assign_key
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1947859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132192,"raw_hex":"e8d3b81d00","size":5,"soft_fail":false,"target_va":5374080272,"va":5372132408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4020616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132192,"raw_hex":"e888593d00","size":5,"soft_fail":false,"target_va":5376152880,"va":5372132259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1026558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132192,"raw_hex":"e8fea90f00","size":5,"soft_fail":false,"target_va":5373158864,"va":5372132301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":61728,"parent_record_offset":61304,"ancestor_record_offset":61304,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":220,"record_offset":61472,"parent_record_offset":61304,"ancestor_record_offset":61304,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":61656,"parent_record_offset":61632,"ancestor_record_offset":61304,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":61632,"parent_record_offset":61592,"ancestor_record_offset":61304,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":61592,"parent_record_offset":61556,"ancestor_record_offset":61304,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":61524,"parent_record_offset":61496,"ancestor_record_offset":61304,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":61496,"parent_record_offset":61472,"ancestor_record_offset":61304,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":220,"record_offset":61556,"parent_record_offset":61472,"ancestor_record_offset":61304,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":61688,"parent_record_offset":61592,"ancestor_record_offset":61304,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$19
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 863
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3300
// 首条位置: {"module":220,"inlinee":13056}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":220,"record_offset":9788,"parent_record_offset":9764,"ancestor_record_offset":9764,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$20
// 记录形式: compiled_procedure; instances=1
// 源码行: 867
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":220,"record_offset":61792,"rva":3423376,"va":5372132496}
// 编译实例 RVAs: 0x343c90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=2
//     target_aliases=8 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1017289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132496,"raw_hex":"e8c9850f00","size":5,"soft_fail":false,"target_va":5373149856,"va":5372132562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1046877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132496,"raw_hex":"e85df90f00","size":5,"soft_fail":false,"target_va":5373179424,"va":5372132542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14051f3d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1947291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132496,"raw_hex":"e89bb61d00","size":5,"soft_fail":false,"target_va":5374079952,"va":5372132656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31460665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132496,"raw_hex":"e8390de001","size":5,"soft_fail":false,"target_va":5403593360,"va":5372132690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-827572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132496,"raw_hex":"e84c5ff3ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5372132623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4020347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132496,"raw_hex":"e87b583d00","size":5,"soft_fail":false,"target_va":5376152880,"va":5372132528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1026289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132496,"raw_hex":"e8f1a80f00","size":5,"soft_fail":false,"target_va":5373158864,"va":5372132570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":62212,"parent_record_offset":61792,"ancestor_record_offset":61792,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":220,"record_offset":61960,"parent_record_offset":61792,"ancestor_record_offset":61792,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":62140,"parent_record_offset":62116,"ancestor_record_offset":61792,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":62116,"parent_record_offset":62080,"ancestor_record_offset":61792,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":62080,"parent_record_offset":62044,"ancestor_record_offset":61792,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":62012,"parent_record_offset":61984,"ancestor_record_offset":61792,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":61984,"parent_record_offset":61960,"ancestor_record_offset":61792,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":220,"record_offset":62044,"parent_record_offset":61960,"ancestor_record_offset":61792,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":62172,"parent_record_offset":62080,"ancestor_record_offset":61792,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$21
// 记录形式: compiled_procedure; instances=1
// 源码行: 891
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":220,"record_offset":62276,"rva":3423680,"va":5372132800}
// 编译实例 RVAs: 0x343dc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2904195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132800,"raw_hex":"e87dafd3ff","size":5,"soft_fail":false,"target_va":5369228752,"va":5372132942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2902220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132800,"raw_hex":"e834b7d3ff","size":5,"soft_fail":false,"target_va":5369230672,"va":5372132887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-834115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132800,"raw_hex":"e8bd45f3ff","size":5,"soft_fail":false,"target_va":5371298848,"va":5372132958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t616514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132800,"raw_hex":"e842680900","size":5,"soft_fail":false,"target_va":5372749712,"va":5372133193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443570
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1046318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132800,"raw_hex":"e82ef70f00","size":5,"soft_fail":false,"target_va":5373179248,"va":5372132925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5396752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132800,"raw_hex":"e810595200","size":5,"soft_fail":false,"target_va":5377529888,"va":5372133131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30845394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132800,"raw_hex":"e8d2a9d601","size":5,"soft_fail":false,"target_va":5402978880,"va":5372133481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_key_bindings::MouseBinding::action_desc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1950969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132800,"raw_hex":"e8f9c41d00","size":5,"soft_fail":false,"target_va":5374083824,"va":5372132850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_key_bindings::MouseBinding::get_mouse_button
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1950792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132800,"raw_hex":"e848c41d00","size":5,"soft_fail":false,"target_va":5374083792,"va":5372132995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_key_bindings::mouse_button_to_string
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1950999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132800,"raw_hex":"e817c51d00","size":5,"soft_fail":false,"target_va":5374084016,"va":5372133012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4019830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372132800,"raw_hex":"e876563d00","size":5,"soft_fail":false,"target_va":5376152880,"va":5372133045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":220,"record_offset":62556,"parent_record_offset":62276,"ancestor_record_offset":62276,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":62460,"parent_record_offset":62276,"ancestor_record_offset":62276,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":62640,"parent_record_offset":62276,"ancestor_record_offset":62276,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":62488,"parent_record_offset":62276,"ancestor_record_offset":62276,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":63232,"parent_record_offset":62276,"ancestor_record_offset":62276,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":220,"record_offset":62524,"parent_record_offset":62276,"ancestor_record_offset":62276,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":62860,"parent_record_offset":62836,"ancestor_record_offset":62276,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":62836,"parent_record_offset":62780,"ancestor_record_offset":62276,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":220,"record_offset":62604,"parent_record_offset":62580,"ancestor_record_offset":62276,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":62808,"parent_record_offset":62780,"ancestor_record_offset":62276,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":62780,"parent_record_offset":62752,"ancestor_record_offset":62276,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":220,"record_offset":62676,"parent_record_offset":62640,"ancestor_record_offset":62276,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":62580,"parent_record_offset":62556,"ancestor_record_offset":62276,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":62704,"parent_record_offset":62640,"ancestor_record_offset":62276,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$22
// 记录形式: compiled_procedure; instances=1
// 源码行: 914
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":220,"record_offset":64908,"rva":3424720,"va":5372133840}
// 编译实例 RVAs: 0x3441d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=2
//     target_aliases=8 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1015921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372133840,"raw_hex":"e871800f00","size":5,"soft_fail":false,"target_va":5373149856,"va":5372133930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1045509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372133840,"raw_hex":"e805f40f00","size":5,"soft_fail":false,"target_va":5373179424,"va":5372133910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-828937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372133840,"raw_hex":"e8f759f3ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5372133988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_key_bindings::KeyBindings::assign_mouse_button
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1946189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372133840,"raw_hex":"e84db21d00","size":5,"soft_fail":false,"target_va":5374080240,"va":5372134046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4018979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372133840,"raw_hex":"e823533d00","size":5,"soft_fail":false,"target_va":5376152880,"va":5372133896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1024921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372133840,"raw_hex":"e899a30f00","size":5,"soft_fail":false,"target_va":5373158864,"va":5372133938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":65332,"parent_record_offset":64908,"ancestor_record_offset":64908,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":220,"record_offset":65076,"parent_record_offset":64908,"ancestor_record_offset":64908,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":65260,"parent_record_offset":65236,"ancestor_record_offset":64908,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":65236,"parent_record_offset":65196,"ancestor_record_offset":64908,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":65196,"parent_record_offset":65160,"ancestor_record_offset":64908,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":65128,"parent_record_offset":65100,"ancestor_record_offset":64908,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":65100,"parent_record_offset":65076,"ancestor_record_offset":64908,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":220,"record_offset":65160,"parent_record_offset":65076,"ancestor_record_offset":64908,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":65292,"parent_record_offset":65196,"ancestor_record_offset":64908,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 964
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3309
// 首条位置: {"module":220,"inlinee":13065}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":220,"record_offset":23100,"parent_record_offset":23076,"ancestor_record_offset":23076,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::_::impl$0::next
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 974
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1620
// 首条位置: {"module":15,"inlinee":5664}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::_::impl$0::previous
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 974
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x161F
// 首条位置: {"module":15,"inlinee":5663}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::impl$17::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 992
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2154
// 首条位置: {"module":73,"inlinee":8532}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":51896,"parent_record_offset":51872,"ancestor_record_offset":51872,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":220,"record_offset":51920,"parent_record_offset":51896,"ancestor_record_offset":51872,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::impl$2::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 996
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1622
// 首条位置: {"module":15,"inlinee":5666}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":18276,"parent_record_offset":18252,"ancestor_record_offset":18252,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::impl$22::eq
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 1001
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2156
// 首条位置: {"module":73,"inlinee":8534}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::impl$24::partial_cmp
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 1001
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3121
// 首条位置: {"module":194,"inlinee":12577}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$60::partial_cmp
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":24116,"parent_record_offset":24092,"ancestor_record_offset":24092,"inlinee":12578,"source_line":1549,"archive":"_pdb/A/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$61::cmp
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":24140,"parent_record_offset":24116,"ancestor_record_offset":24092,"inlinee":12580,"source_line":1565,"archive":"_pdb/A/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::impl$3::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1005
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1625
// 首条位置: {"module":15,"inlinee":5669}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":28088,"parent_record_offset":28064,"ancestor_record_offset":28064,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::impl$4::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1017
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x161A
// 首条位置: {"module":15,"inlinee":5658}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::impl$5::from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1026
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2F31
// 首条位置: {"module":220,"inlinee":12081}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::impl$6::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 1036
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x161D
// 首条位置: {"module":15,"inlinee":5661}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::settings::detect_konami_code
// 记录形式: compiled_procedure; instances=1
// 源码行: 1048
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":170,"record_offset":7288,"rva":2955424,"va":5371664544}
// 编译实例 RVAs: 0x2d18a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_input::button_input::ButtonInput::get_just_pressed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/button_input.rs
//     evidence={"module":170,"record_offset":7432,"parent_record_offset":7288,"ancestor_record_offset":7288,"inlinee":11746,"source_line":287,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] bevy_input::keyboard::impl$28::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/keyboard.rs
//     evidence={"module":170,"record_offset":8484,"parent_record_offset":7288,"ancestor_record_offset":7288,"inlinee":12072,"source_line":200,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":170,"record_offset":8420,"parent_record_offset":7288,"ancestor_record_offset":7288,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] hashbrown::set::impl$24::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/set.rs
//     evidence={"module":170,"record_offset":7836,"parent_record_offset":7288,"ancestor_record_offset":7288,"inlinee":12068,"source_line":1692,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$20::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/keyboard.rs
//     evidence={"module":170,"record_offset":8512,"parent_record_offset":8484,"ancestor_record_offset":7288,"inlinee":12074,"source_line":163,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":170,"record_offset":7712,"parent_record_offset":7684,"ancestor_record_offset":7288,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":170,"record_offset":7772,"parent_record_offset":7740,"ancestor_record_offset":7288,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":170,"record_offset":8112,"parent_record_offset":8084,"ancestor_record_offset":7288,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":170,"record_offset":7656,"parent_record_offset":7632,"ancestor_record_offset":7288,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":170,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":7288,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":170,"record_offset":8312,"parent_record_offset":8288,"ancestor_record_offset":7288,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":170,"record_offset":8448,"parent_record_offset":8420,"ancestor_record_offset":7288,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":170,"record_offset":7516,"parent_record_offset":7488,"ancestor_record_offset":7288,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":170,"record_offset":7488,"parent_record_offset":7460,"ancestor_record_offset":7288,"inlinee":11730,"source_line":652,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":170,"record_offset":7900,"parent_record_offset":7868,"ancestor_record_offset":7288,"inlinee":8353,"source_line":4727,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$74::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":170,"record_offset":7868,"parent_record_offset":7836,"ancestor_record_offset":7288,"inlinee":12070,"source_line":4848,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::next_n
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":170,"record_offset":8288,"parent_record_offset":7968,"ancestor_record_offset":7288,"inlinee":8846,"source_line":561,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawIterRange::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":170,"record_offset":7632,"parent_record_offset":7572,"ancestor_record_offset":7288,"inlinee":8838,"source_line":3755,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawIterRange::next_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":170,"record_offset":7968,"parent_record_offset":7932,"ancestor_record_offset":7288,"inlinee":8845,"source_line":3822,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":170,"record_offset":7544,"parent_record_offset":7516,"ancestor_record_offset":7288,"inlinee":11731,"source_line":1594,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::data_end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":170,"record_offset":7604,"parent_record_offset":7572,"ancestor_record_offset":7288,"inlinee":8856,"source_line":2595,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":170,"record_offset":7572,"parent_record_offset":7544,"ancestor_record_offset":7288,"inlinee":8836,"source_line":2295,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":170,"record_offset":8048,"parent_record_offset":8016,"ancestor_record_offset":7288,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":170,"record_offset":8084,"parent_record_offset":8048,"ancestor_record_offset":7288,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":170,"record_offset":8016,"parent_record_offset":7968,"ancestor_record_offset":7288,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$27::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":170,"record_offset":7932,"parent_record_offset":7900,"ancestor_record_offset":7288,"inlinee":8844,"source_line":4122,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load_aligned
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":170,"record_offset":7684,"parent_record_offset":7632,"ancestor_record_offset":7288,"inlinee":4897,"source_line":60,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty_or_deleted
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":170,"record_offset":7740,"parent_record_offset":7684,"ancestor_record_offset":7288,"inlinee":4894,"source_line":104,"archive":"_pdb/A/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/set.rs
//     evidence={"module":170,"record_offset":7460,"parent_record_offset":7432,"ancestor_record_offset":7288,"inlinee":11729,"source_line":291,"archive":"_pdb/A/modules/00170.jsonl.gz"}
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
// 完整结构及原始类型记录: settings.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::gui::settings::DisplayMode
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode>", "constructor": "toolbar::mei::widgets::tweaker::TweakableEnum", "arguments": ["tiny_glade::gui::settings::DisplayMode"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >", "constructor": "toolbar::mei::widgets::tweaker::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::gui::settings::FpsCap
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::gui::settings::FpsCap>", "constructor": "enum2$", "arguments": ["tiny_glade::gui::settings::FpsCap"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> >", "constructor": "toolbar::mei::widgets::tweaker::TweakableList", "arguments": ["enum2$<tiny_glade::gui::settings::FpsCap>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > >", "constructor": "toolbar::mei::widgets::tweaker::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::gui::settings::LastAppliedVideoSettings
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 34
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>", "constructor": "bevy_ecs::change_detection::ResMut", "arguments": ["tiny_glade::gui::settings::LastAppliedVideoSettings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::run_if<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)>,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)>", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::run_if", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)>", "bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$7::init_state<tiny_glade::gui::settings::LastAppliedVideoSettings>", "constructor": "bevy_ecs::system::system_param::impl$7::init_state", "arguments": ["tiny_glade::gui::settings::LastAppliedVideoSettings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::insert_resource<tiny_glade::gui::settings::LastAppliedVideoSettings>", "constructor": "bevy_app::app::App::insert_resource", "arguments": ["tiny_glade::gui::settings::LastAppliedVideoSettings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::ComponentDescriptor::drop_ptr<tiny_glade::gui::settings::LastAppliedVideoSettings>", "constructor": "bevy_ecs::component::ComponentDescriptor::drop_ptr", "arguments": ["tiny_glade::gui::settings::LastAppliedVideoSettings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::gui::settings::RefreshRateMillihertz
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz>", "constructor": "toolbar::mei::widgets::tweaker::TweakableList", "arguments": ["tiny_glade::gui::settings::RefreshRateMillihertz"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> >", "constructor": "toolbar::mei::widgets::tweaker::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$5::dedup::closure_env$0<tiny_glade::gui::settings::RefreshRateMillihertz,alloc::alloc::Global>", "constructor": "alloc::vec::impl$5::dedup::closure_env$0", "arguments": ["tiny_glade::gui::settings::RefreshRateMillihertz", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::dedup_by<tiny_glade::gui::settings::RefreshRateMillihertz,alloc::alloc::Global,alloc::vec::impl$5::dedup::closure_env$0<tiny_glade::gui::settings::RefreshRateMillihertz,alloc::alloc::Global> >", "constructor": "alloc::vec::Vec::dedup_by", "arguments": ["tiny_glade::gui::settings::RefreshRateMillihertz", "alloc::alloc::Global", "alloc::vec::impl$5::dedup::closure_env$0<tiny_glade::gui::settings::RefreshRateMillihertz,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<tiny_glade::gui::settings::RefreshRateMillihertz,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::render::render_config::VideoMode>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$11> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["tiny_glade::gui::settings::RefreshRateMillihertz", "core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::render::render_config::VideoMode>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$11>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tiny_glade::gui::settings::RefreshRateMillihertz>", "constructor": "ref$", "arguments": ["tiny_glade::gui::settings::RefreshRateMillihertz"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$0<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>", "constructor": "alloc::slice::stable_sort::closure_env$0", "arguments": ["tiny_glade::gui::settings::RefreshRateMillihertz", "bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$1<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>", "constructor": "alloc::slice::stable_sort::closure_env$1", "arguments": ["tiny_glade::gui::settings::RefreshRateMillihertz", "bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$2<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>", "constructor": "alloc::slice::stable_sort::closure_env$2", "arguments": ["tiny_glade::gui::settings::RefreshRateMillihertz", "bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$3<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>", "constructor": "alloc::slice::stable_sort::closure_env$3", "arguments": ["tiny_glade::gui::settings::RefreshRateMillihertz", "bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::merge_sort<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>),alloc::slice::stable_sort::closure_env$0<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$1<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$2<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$3<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)> >", "constructor": "core::slice::sort::merge_sort", "arguments": ["tiny_glade::gui::settings::RefreshRateMillihertz", "bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)", "alloc::slice::stable_sort::closure_env$0<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>", "alloc::slice::stable_sort::closure_env$1<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>", "alloc::slice::stable_sort::closure_env$2<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>", "alloc::slice::stable_sort::closure_env$3<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_left<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>", "constructor": "core::slice::sort::insertion_sort_shift_left", "arguments": ["tiny_glade::gui::settings::RefreshRateMillihertz", "bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::gui::settings::ResetTutorialUiState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 30
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["tiny_glade::gui::settings::ResetTutorialUiState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::run_if<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)>,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)>", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::run_if", "arguments": ["void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)>", "bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::gui::settings::Resolution
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution>", "constructor": "toolbar::mei::widgets::tweaker::TweakableList", "arguments": ["tiny_glade::gui::settings::Resolution"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> >", "constructor": "toolbar::mei::widgets::tweaker::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::gui::settings::SettingsMenuPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$9::add_to_app<bevy_app::plugin::sealed::PluginMarker,country_core::startup::MinimalStartupPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::logo_animation::AnimatedLogo,bevy_app::plugin::sealed::PluginMarker,country_core::resources::loading_screen::LoadingScreen,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::SplashScreenPlugin>", "constructor": "bevy_app::plugin::sealed::impl$9::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "country_core::startup::MinimalStartupPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::logo_animation::AnimatedLogo", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::loading_screen::LoadingScreen", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::vermeer::VermeerPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$14::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$14::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: settings.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: settings.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 17; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::render::render_config::LastKnownGraphicsDevice>,ref$<country_core::render::render_config::LastKnownGraphicsDevice>)
// 候选参数类型: ["ref$<country_core::render::render_config::LastKnownGraphicsDevice>", "ref$<country_core::render::render_config::LastKnownGraphicsDevice>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<winit::monitor::VideoMode>,ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>", "ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<alloc::string::String> >,ref$<ref$<alloc::string::String> >)
// 候选参数类型: ["ref$<ref$<alloc::string::String> >", "ref$<ref$<alloc::string::String> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)
// 候选参数类型: ["ref$<tiny_glade::gui::settings::RefreshRateMillihertz>", "ref$<tiny_glade::gui::settings::RefreshRateMillihertz>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)
// 候选参数类型: ["ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >", "ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)
// 候选参数类型: ["tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >", "bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>", "bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>", "bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>", "bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>", "bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>", "bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>", "bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>", "bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::system::system_param::Local<usize>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::system::system_param::Local<usize>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<usize>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::LastAppliedVideoSettings::needs_apply
// 信息 ID: 89fc47e7fc366dcbf267df47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x34318f -> 0x2d0b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::LastAppliedVideoSettings::populate_empty_from
// 信息 ID: 55ebf12a80d48d3adc3e1a43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings; module=170, inline_record=3204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::_::impl$0::next
// 信息 ID: d023950bc4fad9b510f36bf8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >; module=15, inline_record=9504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::_::impl$0::previous
// 信息 ID: 7b39b31e7586f13faf615cae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >; module=15, inline_record=8252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::detect_konami_code
// 信息 ID: b7ed9e505a1cd8baa7ba8163
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d75d862fac6de52c3786f97e: void (*)(bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::system::system_param::Local<usize>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::system::system_param::Local<usize>),void (*)(bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::system::system_param::Local<usize>)> > @ 0xae93c -> 0x2d18a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::system::system_param::Local<usize>),void (*)(bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::system::system_param::Local<usize>)> @ 0x1dcb46 -> 0x2d18a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings
// 信息 ID: acebd3e9d0a9c548ad2e5c1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 712f051ee663f3ec91d3c80c: void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> > @ 0xc5b06 -> 0x2d0c70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,bevy_ecs::event::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<tiny_glade::gui::settings::ResetTutorialUiState>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiKeyBindingsState>,bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> @ 0x1dd74a -> 0x2d0c70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$0
// 信息 ID: 7ea29ea32338f6b71fe4a9b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings; module=170, inline_record=6016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$1
// 信息 ID: a05848cf21dbebaddb7511fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$1 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$1,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$1,ref$<str$> >; module=220, inline_record=15136; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$1,ref$<str$> >; module=220, inline_record=15696; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$1,ref$<str$> >; module=220, inline_record=15972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$2
// 信息 ID: 08180b79b1559bf48dde7545
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$2 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$2,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$2,ref$<str$> >; module=220, inline_record=19864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$3
// 信息 ID: 5e9031bd8a6128e899286c43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$3 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$3,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$3,ref$<str$> > @ 0x33bf2f -> 0x3408d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4
// 信息 ID: ac1fc48d27e1d1e4bbb87045
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$4 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$4,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$4,ref$<str$> > @ 0x33cf8a -> 0x340d70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$0
// 信息 ID: ccd491f69c167d26a9cbe2af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4; module=220, inline_record=47312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$0::closure$0
// 信息 ID: ca95b129ea3e747788a8c505
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4; module=220, inline_record=47460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$1
// 信息 ID: 704ed386a8e537fb894c6d7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1 <- core::iter::adapters::map::impl$2::fold<alloc::string::String,core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1 <- core::iter::adapters::map::impl$2::fold<alloc::string::String,core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<alloc::string::String,core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1> > > >; module=21, inline_record=16756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$10
// 信息 ID: f08e2ddb00c383748b8a8478
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4; module=220, inline_record=53076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$11
// 信息 ID: 92d4efb215dc7bc92c1d8835
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$11 <- alloc::vec::spec_from_iter::impl$0::from_iter<tiny_glade::gui::settings::RefreshRateMillihertz,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::render::render_config::VideoMode>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$11> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tiny_glade::gui::settings::RefreshRateMillihertz,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::render::render_config::VideoMode>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$11> >; module=73, inline_record=70448; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tiny_glade::gui::settings::RefreshRateMillihertz,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::render::render_config::VideoMode>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$11> >; module=73, inline_record=71112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$12
// 信息 ID: 1aacc9e5556e69f1d097ceca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4; module=220, inline_record=53448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$13
// 信息 ID: 506225b4eb6d5d9d1420a175
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4; module=220, inline_record=53504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$14
// 信息 ID: a60a2523f92cb249f8310abc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4; module=220, inline_record=53728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$16
// 信息 ID: 19186b1706ca89ba9552e1b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4; module=220, inline_record=47104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$17
// 信息 ID: 20ee5b62df8ef18a11d30d49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$17 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$17,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$17,ref$<str$> > @ 0x33c6a5 -> 0x3436c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$17::closure$0
// 信息 ID: 9a83a742325adcfb99102cb7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$17; module=220, inline_record=58660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$17::closure$1
// 信息 ID: 51ccfc3ca38bc87476d6718b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$17; module=220, inline_record=58852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$18
// 信息 ID: 6a3a38912e2df416a3a8f9b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$18 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$18,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$18,ref$<str$> > @ 0x33dcf5 -> 0x343b60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$19
// 信息 ID: c3dff0ae82b7a9c3206ec1a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$19 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$19,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$19,ref$<str$> >; module=220, inline_record=9764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$2
// 信息 ID: 3a34c6e774120ce0e60a8e85
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<ref$<alloc::string::String>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::window::AvailableMonitor>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<alloc::string::String>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::window::AvailableMonitor>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$2> >; module=73, inline_record=91852; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<alloc::string::String>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::window::AvailableMonitor>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$2> >; module=73, inline_record=92476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$20
// 信息 ID: 13845687468d07daec8e0e0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$20 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$20,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$20,ref$<str$> > @ 0x33ae95 -> 0x343c90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$21
// 信息 ID: 0d4ac0d0c754c9e413b7e02e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$21 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$21,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$21,ref$<str$> > @ 0x33ab05 -> 0x343dc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$22
// 信息 ID: 9817da73af353aa2daeb17c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$22 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$22,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$22,ref$<str$> > @ 0x33b7d5 -> 0x3441d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$3
// 信息 ID: a51444f34c0da8bd78af77a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4; module=220, inline_record=49568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$4
// 信息 ID: cafd91536cc761a65dbf1d86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$4 <- core::slice::iter::impl$181::find<country_core::resources::window::AvailableMonitor,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$4>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::slice::iter::impl$181::find<country_core::resources::window::AvailableMonitor,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$4>; module=220, inline_record=23712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$5
// 信息 ID: 3144ffec4394d388fe617d8d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4; module=220, inline_record=50288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$6
// 信息 ID: 2046642e6c0266322832d173
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$6 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::render::render_config::VideoMode,core::iter::adapters::flatten::FlatMap<core::option::IntoIter<ref$<country_core::resources::window::AvailableMonitor> >,core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::render::render_config::VideoMode> >,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$6> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::render::render_config::VideoMode,core::iter::adapters::flatten::FlatMap<core::option::IntoIter<ref$<country_core::resources::window::AvailableMonitor> >,core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::render::render_config::VideoMode> >,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$6> >; module=73, inline_record=75376; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::render::render_config::VideoMode,core::iter::adapters::flatten::FlatMap<core::option::IntoIter<ref$<country_core::resources::window::AvailableMonitor> >,core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::render::render_config::VideoMode> >,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$6> >; module=73, inline_record=77636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$8
// 信息 ID: 4772257c8fcf4dfe14766cf7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4; module=220, inline_record=52560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$8::closure$0
// 信息 ID: ce4d2ee96d3245cd417d446c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4; module=220, inline_record=52612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$9
// 信息 ID: 4eff35b94f5791fadbea553e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4; module=220, inline_record=52856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$5
// 信息 ID: 4d8485993aa898fd52432f6a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::gui::settings::do_gui_escape_menu_settings", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$5 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$5,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$5,ref$<str$> >; module=220, inline_record=23076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::impl$0::build
// 信息 ID: 2ca8f535bad36967e22c418b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::impl$17::eq
// 信息 ID: f930d5ee988ea5581d401994
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4; module=220, inline_record=51872; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4; module=220, inline_record=52012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::impl$1::populate_empty_from::closure$0
// 信息 ID: e6c82ca97495bde0ec9abcbc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::settings::impl$1::populate_empty_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings; module=170, inline_record=3276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::impl$22::eq
// 信息 ID: e2860b12f279a81b55dc4132
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::Vec::dedup_by<tiny_glade::gui::settings::RefreshRateMillihertz,alloc::alloc::Global,alloc::vec::impl$5::dedup::closure_env$0<tiny_glade::gui::settings::RefreshRateMillihertz,alloc::alloc::Global> >; module=73, inline_record=16392; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::dedup_by<tiny_glade::gui::settings::RefreshRateMillihertz,alloc::alloc::Global,alloc::vec::impl$5::dedup::closure_env$0<tiny_glade::gui::settings::RefreshRateMillihertz,alloc::alloc::Global> >; module=73, inline_record=16472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::impl$24::partial_cmp
// 信息 ID: a4f49a475085dbe8b6a95757
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d8d7c68aeaa0dfaed431f018: bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>),alloc::slice::stable_sort::closure_env$0<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$1<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$2<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$3<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)> >; module=194, inline_record=24092; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>),alloc::slice::stable_sort::closure_env$0<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$1<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$2<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$3<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)> >; module=194, inline_record=24232; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>),alloc::slice::stable_sort::closure_env$0<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$1<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$2<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$3<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)> >; module=194, inline_record=24372; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>),alloc::slice::stable_sort::closure_env$0<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$1<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$2<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$3<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)> >; module=194, inline_record=25432; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>),alloc::slice::stable_sort::closure_env$0<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$1<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$2<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>,alloc::slice::stable_sort::closure_env$3<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)> >; module=194, inline_record=25792; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>; module=194, inline_record=41128; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<tiny_glade::gui::settings::RefreshRateMillihertz,bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)>; module=194, inline_record=41416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::impl$2::fmt
// 信息 ID: 8b710c4b0f0ea9de1ecddf51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> >; module=15, inline_record=18252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::impl$3::fmt
// 信息 ID: 851bb8703e3cbe1374abe41f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> >; module=15, inline_record=28064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::impl$4::fmt
// 信息 ID: de861721e3be78ca5cc9589d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > >; module=15, inline_record=5220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::impl$5::from
// 信息 ID: 0a8ac1c45daa96f6df713da8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4; module=220, inline_record=49224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::impl$6::fmt
// 信息 ID: ee3e0ffb2f4adfcf1f4b0c5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >; module=15, inline_record=7840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::settings::impl$7::default
// 信息 ID: a99f5be2b2f02b4101781650
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::impl$0::build; module=170, inline_record=364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
