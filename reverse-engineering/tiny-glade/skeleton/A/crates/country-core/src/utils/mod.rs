// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/utils/mod.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: fcf6ba4b58cc1cb90d1ee3e06d0218123efba74eaf2e6087a3091d59e9be4757
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 105, 127, 136, 197, 387, 388, 389, 394, 395, 397, 400, 402, 411, 415, 421, 430, 433, 441, 442, 448, 449, 451, 454, 456, 459, 462, 463, 469, 473, 485, 490, 493, 495, 498, 1302, 1403
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::convert_function_expr_to_title::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x684F
// 首条位置: {"module":469,"inlinee":26703}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::get_res
// 记录形式: inline_instance, inlinee_declaration; instances=201
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x26CC
// 首条位置: {"module":127,"inlinee":9932}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=111
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":19540,"parent_record_offset":19504,"ancestor_record_offset":19504,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":21236,"parent_record_offset":19504,"ancestor_record_offset":19504,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=111
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":127,"record_offset":19576,"parent_record_offset":19540,"ancestor_record_offset":19504,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=111
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":21188,"parent_record_offset":21152,"ancestor_record_offset":19504,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=111
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":21120,"parent_record_offset":20680,"ancestor_record_offset":19504,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=111
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":21152,"parent_record_offset":21120,"ancestor_record_offset":19504,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=111
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":20716,"parent_record_offset":20680,"ancestor_record_offset":19504,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=111
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":20772,"parent_record_offset":20744,"ancestor_record_offset":19504,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=111
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":20744,"parent_record_offset":20716,"ancestor_record_offset":19504,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=111
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":20876,"parent_record_offset":20804,"ancestor_record_offset":19504,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=111
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":20972,"parent_record_offset":20944,"ancestor_record_offset":19504,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=111
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":20680,"parent_record_offset":19540,"ancestor_record_offset":19504,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":127,"record_offset":20384,"parent_record_offset":20356,"ancestor_record_offset":19504,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":20440,"parent_record_offset":20412,"ancestor_record_offset":19504,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":20356,"parent_record_offset":20328,"ancestor_record_offset":19504,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":19792,"parent_record_offset":19764,"ancestor_record_offset":19504,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=222
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":19888,"parent_record_offset":19860,"ancestor_record_offset":19504,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":127,"record_offset":19820,"parent_record_offset":19792,"ancestor_record_offset":19504,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":127,"record_offset":20008,"parent_record_offset":19980,"ancestor_record_offset":19504,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":20900,"parent_record_offset":20876,"ancestor_record_offset":19504,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":20644,"parent_record_offset":19576,"ancestor_record_offset":19504,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":20944,"parent_record_offset":20744,"ancestor_record_offset":19504,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":127,"record_offset":21064,"parent_record_offset":21036,"ancestor_record_offset":19504,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":20260,"parent_record_offset":20232,"ancestor_record_offset":19504,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=222
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":20168,"parent_record_offset":20140,"ancestor_record_offset":19504,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":127,"record_offset":20804,"parent_record_offset":20772,"ancestor_record_offset":19504,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":20844,"parent_record_offset":20804,"ancestor_record_offset":19504,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":21036,"parent_record_offset":21000,"ancestor_record_offset":19504,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":127,"record_offset":20412,"parent_record_offset":20384,"ancestor_record_offset":19504,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=68
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":127,"record_offset":20328,"parent_record_offset":20304,"ancestor_record_offset":19504,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=43
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":389,"record_offset":22072,"parent_record_offset":22048,"ancestor_record_offset":21344,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=68
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":19604,"parent_record_offset":19576,"ancestor_record_offset":19504,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=43
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":389,"record_offset":21424,"parent_record_offset":21400,"ancestor_record_offset":21344,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=68
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":19632,"parent_record_offset":19604,"ancestor_record_offset":19504,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=43
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":389,"record_offset":21448,"parent_record_offset":21424,"ancestor_record_offset":21344,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=68
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":20304,"parent_record_offset":20084,"ancestor_record_offset":19504,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=43
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":389,"record_offset":22048,"parent_record_offset":21852,"ancestor_record_offset":21344,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=111
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":20204,"parent_record_offset":20108,"ancestor_record_offset":19504,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=111
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":20140,"parent_record_offset":20108,"ancestor_record_offset":19504,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=111
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":20592,"parent_record_offset":19720,"ancestor_record_offset":19504,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=111
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":20108,"parent_record_offset":20084,"ancestor_record_offset":19504,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=111
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":19692,"parent_record_offset":19664,"ancestor_record_offset":19504,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=111
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":19664,"parent_record_offset":19632,"ancestor_record_offset":19504,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=111
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":19720,"parent_record_offset":19692,"ancestor_record_offset":19504,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=111
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":19952,"parent_record_offset":19924,"ancestor_record_offset":19504,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=111
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":19980,"parent_record_offset":19952,"ancestor_record_offset":19504,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=111
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":20048,"parent_record_offset":19924,"ancestor_record_offset":19504,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=111
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":19924,"parent_record_offset":19720,"ancestor_record_offset":19504,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=111
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":20084,"parent_record_offset":19720,"ancestor_record_offset":19504,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=111
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":19764,"parent_record_offset":19720,"ancestor_record_offset":19504,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=222
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":19860,"parent_record_offset":19720,"ancestor_record_offset":19504,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=111
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":20496,"parent_record_offset":19720,"ancestor_record_offset":19504,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=111
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":127,"record_offset":21000,"parent_record_offset":20972,"ancestor_record_offset":19504,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::get_res_mut
// 记录形式: inline_instance, inlinee_declaration; instances=32
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x26C9
// 首条位置: {"module":127,"inlinee":9929}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":17516,"parent_record_offset":17484,"ancestor_record_offset":17484,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":19464,"parent_record_offset":17484,"ancestor_record_offset":17484,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":127,"record_offset":17548,"parent_record_offset":17516,"ancestor_record_offset":17484,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":19244,"parent_record_offset":19216,"ancestor_record_offset":17484,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":19216,"parent_record_offset":19184,"ancestor_record_offset":17484,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":19152,"parent_record_offset":19120,"ancestor_record_offset":17484,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":19092,"parent_record_offset":18652,"ancestor_record_offset":17484,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":19120,"parent_record_offset":19092,"ancestor_record_offset":17484,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":18688,"parent_record_offset":18652,"ancestor_record_offset":17484,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":19184,"parent_record_offset":19120,"ancestor_record_offset":17484,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":18744,"parent_record_offset":18716,"ancestor_record_offset":17484,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":18716,"parent_record_offset":18688,"ancestor_record_offset":17484,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":18848,"parent_record_offset":18776,"ancestor_record_offset":17484,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":18944,"parent_record_offset":18916,"ancestor_record_offset":17484,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::last_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":19324,"parent_record_offset":19292,"ancestor_record_offset":17484,"inlinee":9923,"source_line":2096,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":19356,"parent_record_offset":19292,"ancestor_record_offset":17484,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=15
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":24652,"parent_record_offset":24016,"ancestor_record_offset":22848,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":18652,"parent_record_offset":17516,"ancestor_record_offset":17484,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::last_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":19292,"parent_record_offset":18652,"ancestor_record_offset":17484,"inlinee":9922,"source_line":268,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":127,"record_offset":18356,"parent_record_offset":18328,"ancestor_record_offset":17484,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":18412,"parent_record_offset":18384,"ancestor_record_offset":17484,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":18328,"parent_record_offset":18300,"ancestor_record_offset":17484,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":17764,"parent_record_offset":17736,"ancestor_record_offset":17484,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":17860,"parent_record_offset":17832,"ancestor_record_offset":17484,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":127,"record_offset":17792,"parent_record_offset":17764,"ancestor_record_offset":17484,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":127,"record_offset":17980,"parent_record_offset":17952,"ancestor_record_offset":17484,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":18872,"parent_record_offset":18848,"ancestor_record_offset":17484,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":18616,"parent_record_offset":17548,"ancestor_record_offset":17484,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":18916,"parent_record_offset":18716,"ancestor_record_offset":17484,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":127,"record_offset":19036,"parent_record_offset":19008,"ancestor_record_offset":17484,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":18232,"parent_record_offset":18204,"ancestor_record_offset":17484,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":18140,"parent_record_offset":18112,"ancestor_record_offset":17484,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":127,"record_offset":18776,"parent_record_offset":18744,"ancestor_record_offset":17484,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":18816,"parent_record_offset":18776,"ancestor_record_offset":17484,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":19008,"parent_record_offset":18972,"ancestor_record_offset":17484,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":19384,"parent_record_offset":19356,"ancestor_record_offset":17484,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":19412,"parent_record_offset":19384,"ancestor_record_offset":17484,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":127,"record_offset":18384,"parent_record_offset":18356,"ancestor_record_offset":17484,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":127,"record_offset":18300,"parent_record_offset":18276,"ancestor_record_offset":17484,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":17576,"parent_record_offset":17548,"ancestor_record_offset":17484,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":17604,"parent_record_offset":17576,"ancestor_record_offset":17484,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":18276,"parent_record_offset":18056,"ancestor_record_offset":17484,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":18176,"parent_record_offset":18080,"ancestor_record_offset":17484,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":18112,"parent_record_offset":18080,"ancestor_record_offset":17484,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":18564,"parent_record_offset":17692,"ancestor_record_offset":17484,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":18080,"parent_record_offset":18056,"ancestor_record_offset":17484,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":17664,"parent_record_offset":17636,"ancestor_record_offset":17484,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":17636,"parent_record_offset":17604,"ancestor_record_offset":17484,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":17692,"parent_record_offset":17664,"ancestor_record_offset":17484,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":17924,"parent_record_offset":17896,"ancestor_record_offset":17484,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":17952,"parent_record_offset":17924,"ancestor_record_offset":17484,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":18020,"parent_record_offset":17896,"ancestor_record_offset":17484,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":17896,"parent_record_offset":17692,"ancestor_record_offset":17484,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":18056,"parent_record_offset":17692,"ancestor_record_offset":17484,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":17736,"parent_record_offset":17692,"ancestor_record_offset":17484,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=32
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":17832,"parent_record_offset":17692,"ancestor_record_offset":17484,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":18468,"parent_record_offset":17692,"ancestor_record_offset":17484,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":127,"record_offset":18972,"parent_record_offset":18944,"ancestor_record_offset":17484,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::get_last_event<country_core::resources::mouse_capture::OsCursorVisibilityCmd>
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":105,"record_offset":2888,"rva":1527664,"va":5370236784}
// 编译实例 RVAs: 0x174f70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$core..iter..adapters..chain..Chain$LT$A$C$B$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::fold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1322149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370236784,"raw_hex":"e8a52c1400","size":5,"soft_fail":false,"target_va":5371559344,"va":5370237190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32741615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370236784,"raw_hex":"e8ef98f301","size":5,"soft_fail":false,"target_va":5402978944,"va":5370237324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":105,"record_offset":4812,"parent_record_offset":2888,"ancestor_record_offset":2888,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::Events::drain
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":105,"record_offset":4868,"parent_record_offset":2888,"ancestor_record_offset":2888,"inlinee":9924,"source_line":342,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":105,"record_offset":3088,"parent_record_offset":2888,"ancestor_record_offset":2888,"inlinee":7122,"source_line":1593,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":105,"record_offset":5408,"parent_record_offset":2888,"ancestor_record_offset":2888,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":105,"record_offset":5464,"parent_record_offset":2888,"ancestor_record_offset":2888,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":4992,"parent_record_offset":4968,"ancestor_record_offset":2888,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":4968,"parent_record_offset":4932,"ancestor_record_offset":2888,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":4932,"parent_record_offset":4868,"ancestor_record_offset":2888,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":5024,"parent_record_offset":4932,"ancestor_record_offset":2888,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":5052,"parent_record_offset":4932,"ancestor_record_offset":2888,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":105,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":2888,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":105,"record_offset":3140,"parent_record_offset":3112,"ancestor_record_offset":2888,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::reset_start_event_count
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":105,"record_offset":4900,"parent_record_offset":4868,"ancestor_record_offset":2888,"inlinee":9925,"source_line":316,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":105,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":2888,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":105,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":2888,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":105,"record_offset":4568,"parent_record_offset":4536,"ancestor_record_offset":2888,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":105,"record_offset":4512,"parent_record_offset":4120,"ancestor_record_offset":2888,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":105,"record_offset":4536,"parent_record_offset":4512,"ancestor_record_offset":2888,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":105,"record_offset":4152,"parent_record_offset":4120,"ancestor_record_offset":2888,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":105,"record_offset":4596,"parent_record_offset":4536,"ancestor_record_offset":2888,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":105,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":2888,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":105,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":2888,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":105,"record_offset":4292,"parent_record_offset":4228,"ancestor_record_offset":2888,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":105,"record_offset":4380,"parent_record_offset":4356,"ancestor_record_offset":2888,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":105,"record_offset":4712,"parent_record_offset":4688,"ancestor_record_offset":2888,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":105,"record_offset":4688,"parent_record_offset":4120,"ancestor_record_offset":2888,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":105,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":2888,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":105,"record_offset":4120,"parent_record_offset":3112,"ancestor_record_offset":2888,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":105,"record_offset":3852,"parent_record_offset":3828,"ancestor_record_offset":2888,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":105,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":2888,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":105,"record_offset":3828,"parent_record_offset":3804,"ancestor_record_offset":2888,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":105,"record_offset":3328,"parent_record_offset":3304,"ancestor_record_offset":2888,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":105,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":2888,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":105,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":2888,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":105,"record_offset":3352,"parent_record_offset":3328,"ancestor_record_offset":2888,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":105,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":2888,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":105,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":2888,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":105,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":2888,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":105,"record_offset":4088,"parent_record_offset":3140,"ancestor_record_offset":2888,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":105,"record_offset":4356,"parent_record_offset":4176,"ancestor_record_offset":2888,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":105,"record_offset":4460,"parent_record_offset":4436,"ancestor_record_offset":2888,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":105,"record_offset":5128,"parent_record_offset":5104,"ancestor_record_offset":2888,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":105,"record_offset":3740,"parent_record_offset":3716,"ancestor_record_offset":2888,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":105,"record_offset":3660,"parent_record_offset":3636,"ancestor_record_offset":2888,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":4228,"parent_record_offset":4200,"ancestor_record_offset":2888,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":5080,"parent_record_offset":4932,"ancestor_record_offset":2888,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":4264,"parent_record_offset":4228,"ancestor_record_offset":2888,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":4436,"parent_record_offset":4404,"ancestor_record_offset":2888,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":105,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":2888,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":105,"record_offset":4736,"parent_record_offset":4712,"ancestor_record_offset":2888,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":105,"record_offset":4760,"parent_record_offset":4736,"ancestor_record_offset":2888,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":105,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":2888,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_last_event::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":105,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":2888,"inlinee":9927,"source_line":50,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":105,"record_offset":3804,"parent_record_offset":3780,"ancestor_record_offset":2888,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":105,"record_offset":3164,"parent_record_offset":3140,"ancestor_record_offset":2888,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":105,"record_offset":3188,"parent_record_offset":3164,"ancestor_record_offset":2888,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":105,"record_offset":3780,"parent_record_offset":3584,"ancestor_record_offset":2888,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":3692,"parent_record_offset":3608,"ancestor_record_offset":2888,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":3636,"parent_record_offset":3608,"ancestor_record_offset":2888,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":4036,"parent_record_offset":3264,"ancestor_record_offset":2888,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":3608,"parent_record_offset":3584,"ancestor_record_offset":2888,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":3240,"parent_record_offset":3216,"ancestor_record_offset":2888,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":3216,"parent_record_offset":3188,"ancestor_record_offset":2888,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":3264,"parent_record_offset":3240,"ancestor_record_offset":2888,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":105,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":2888,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":105,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":2888,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":105,"record_offset":3552,"parent_record_offset":3444,"ancestor_record_offset":2888,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":105,"record_offset":3444,"parent_record_offset":3264,"ancestor_record_offset":2888,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":3584,"parent_record_offset":3264,"ancestor_record_offset":2888,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":105,"record_offset":3304,"parent_record_offset":3264,"ancestor_record_offset":2888,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":105,"record_offset":3388,"parent_record_offset":3264,"ancestor_record_offset":2888,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":105,"record_offset":3952,"parent_record_offset":3264,"ancestor_record_offset":2888,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":105,"record_offset":4404,"parent_record_offset":4380,"ancestor_record_offset":2888,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::get_last_event<enum2$<country_core::resources::mouse_capture::OsCursorCaptureCmd> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":105,"record_offset":5552,"rva":1528224,"va":5370237344}
// 编译实例 RVAs: 0x1751a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$core..iter..adapters..chain..Chain$LT$A$C$B$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::fold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1321131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370237344,"raw_hex":"e8ab281400","size":5,"soft_fail":false,"target_va":5371558896,"va":5370237760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32741044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370237344,"raw_hex":"e8b496f301","size":5,"soft_fail":false,"target_va":5402978944,"va":5370237895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":105,"record_offset":7480,"parent_record_offset":5552,"ancestor_record_offset":5552,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::Events::drain
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":105,"record_offset":7536,"parent_record_offset":5552,"ancestor_record_offset":5552,"inlinee":9924,"source_line":342,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":105,"record_offset":5756,"parent_record_offset":5552,"ancestor_record_offset":5552,"inlinee":7122,"source_line":1593,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":105,"record_offset":8076,"parent_record_offset":5552,"ancestor_record_offset":5552,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":105,"record_offset":8132,"parent_record_offset":5552,"ancestor_record_offset":5552,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":5552,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":7636,"parent_record_offset":7600,"ancestor_record_offset":5552,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":7600,"parent_record_offset":7536,"ancestor_record_offset":5552,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":7692,"parent_record_offset":7600,"ancestor_record_offset":5552,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":7720,"parent_record_offset":7600,"ancestor_record_offset":5552,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":105,"record_offset":7504,"parent_record_offset":7480,"ancestor_record_offset":5552,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":105,"record_offset":5808,"parent_record_offset":5780,"ancestor_record_offset":5552,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::reset_start_event_count
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":105,"record_offset":7568,"parent_record_offset":7536,"ancestor_record_offset":5552,"inlinee":9925,"source_line":316,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":105,"record_offset":7312,"parent_record_offset":7288,"ancestor_record_offset":5552,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":105,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":5552,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":105,"record_offset":7236,"parent_record_offset":7204,"ancestor_record_offset":5552,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":105,"record_offset":7180,"parent_record_offset":6788,"ancestor_record_offset":5552,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":105,"record_offset":7204,"parent_record_offset":7180,"ancestor_record_offset":5552,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":105,"record_offset":6820,"parent_record_offset":6788,"ancestor_record_offset":5552,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":105,"record_offset":7264,"parent_record_offset":7204,"ancestor_record_offset":5552,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":105,"record_offset":6868,"parent_record_offset":6844,"ancestor_record_offset":5552,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":105,"record_offset":6844,"parent_record_offset":6820,"ancestor_record_offset":5552,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":105,"record_offset":6960,"parent_record_offset":6896,"ancestor_record_offset":5552,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":105,"record_offset":7048,"parent_record_offset":7024,"ancestor_record_offset":5552,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":105,"record_offset":7380,"parent_record_offset":7356,"ancestor_record_offset":5552,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":105,"record_offset":7356,"parent_record_offset":6788,"ancestor_record_offset":5552,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":105,"record_offset":5780,"parent_record_offset":5756,"ancestor_record_offset":5552,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":105,"record_offset":6788,"parent_record_offset":5780,"ancestor_record_offset":5552,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":105,"record_offset":6520,"parent_record_offset":6496,"ancestor_record_offset":5552,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":105,"record_offset":6568,"parent_record_offset":6544,"ancestor_record_offset":5552,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":105,"record_offset":6496,"parent_record_offset":6472,"ancestor_record_offset":5552,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":105,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":5552,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":105,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":5552,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":105,"record_offset":8180,"parent_record_offset":8156,"ancestor_record_offset":5552,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":105,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":5552,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":105,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":5552,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":105,"record_offset":6184,"parent_record_offset":6160,"ancestor_record_offset":5552,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":105,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":5552,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":105,"record_offset":6756,"parent_record_offset":5808,"ancestor_record_offset":5552,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":105,"record_offset":7024,"parent_record_offset":6844,"ancestor_record_offset":5552,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":105,"record_offset":7128,"parent_record_offset":7104,"ancestor_record_offset":5552,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":105,"record_offset":7796,"parent_record_offset":7772,"ancestor_record_offset":5552,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":105,"record_offset":6408,"parent_record_offset":6384,"ancestor_record_offset":5552,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":105,"record_offset":6328,"parent_record_offset":6304,"ancestor_record_offset":5552,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":6896,"parent_record_offset":6868,"ancestor_record_offset":5552,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":7748,"parent_record_offset":7600,"ancestor_record_offset":5552,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":6932,"parent_record_offset":6896,"ancestor_record_offset":5552,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":7104,"parent_record_offset":7072,"ancestor_record_offset":5552,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":105,"record_offset":7772,"parent_record_offset":7748,"ancestor_record_offset":5552,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":105,"record_offset":7404,"parent_record_offset":7380,"ancestor_record_offset":5552,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":105,"record_offset":7428,"parent_record_offset":7404,"ancestor_record_offset":5552,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":105,"record_offset":6544,"parent_record_offset":6520,"ancestor_record_offset":5552,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_last_event::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":105,"record_offset":8156,"parent_record_offset":8132,"ancestor_record_offset":5552,"inlinee":9927,"source_line":50,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":105,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":5552,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":105,"record_offset":5832,"parent_record_offset":5808,"ancestor_record_offset":5552,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":105,"record_offset":5856,"parent_record_offset":5832,"ancestor_record_offset":5552,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":105,"record_offset":6448,"parent_record_offset":6252,"ancestor_record_offset":5552,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":6360,"parent_record_offset":6276,"ancestor_record_offset":5552,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":6304,"parent_record_offset":6276,"ancestor_record_offset":5552,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":6704,"parent_record_offset":5932,"ancestor_record_offset":5552,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":6276,"parent_record_offset":6252,"ancestor_record_offset":5552,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":5908,"parent_record_offset":5884,"ancestor_record_offset":5552,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":5884,"parent_record_offset":5856,"ancestor_record_offset":5552,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":5552,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":105,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":5552,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":105,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":5552,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":105,"record_offset":6220,"parent_record_offset":6112,"ancestor_record_offset":5552,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":105,"record_offset":6112,"parent_record_offset":5932,"ancestor_record_offset":5552,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":6252,"parent_record_offset":5932,"ancestor_record_offset":5552,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":105,"record_offset":5972,"parent_record_offset":5932,"ancestor_record_offset":5552,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":105,"record_offset":6056,"parent_record_offset":5932,"ancestor_record_offset":5552,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":105,"record_offset":6620,"parent_record_offset":5932,"ancestor_record_offset":5552,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":105,"record_offset":7072,"parent_record_offset":7048,"ancestor_record_offset":5552,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::get_last_event::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x26C7
// 首条位置: {"module":105,"inlinee":9927}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":105,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::drain_events<enum2$<country_core::resources::window::OsWindowCmd> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":105,"record_offset":232,"rva":1527232,"va":5370236352}
// 编译实例 RVAs: 0x174dc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e7db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-577952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370236352,"raw_hex":"e8602ef7ff","size":5,"soft_fail":false,"target_va":5369658800,"va":5370236747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32742102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370236352,"raw_hex":"e8d69af301","size":5,"soft_fail":false,"target_va":5402978880,"va":5370236773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":105,"record_offset":2144,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::Events::drain
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":105,"record_offset":2200,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":9924,"source_line":342,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":105,"record_offset":420,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":7122,"source_line":1593,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":105,"record_offset":2800,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":105,"record_offset":2856,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":2328,"parent_record_offset":2304,"ancestor_record_offset":232,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":2304,"parent_record_offset":2268,"ancestor_record_offset":232,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":2268,"parent_record_offset":2200,"ancestor_record_offset":232,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":2360,"parent_record_offset":2268,"ancestor_record_offset":232,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":2388,"parent_record_offset":2268,"ancestor_record_offset":232,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":232,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":105,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":232,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":105,"record_offset":472,"parent_record_offset":444,"ancestor_record_offset":232,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::reset_start_event_count
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":105,"record_offset":2236,"parent_record_offset":2200,"ancestor_record_offset":232,"inlinee":9925,"source_line":316,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":105,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":232,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":105,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":232,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":105,"record_offset":1900,"parent_record_offset":1868,"ancestor_record_offset":232,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":105,"record_offset":1844,"parent_record_offset":1452,"ancestor_record_offset":232,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":105,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":232,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":105,"record_offset":1484,"parent_record_offset":1452,"ancestor_record_offset":232,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":105,"record_offset":1928,"parent_record_offset":1868,"ancestor_record_offset":232,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":105,"record_offset":1532,"parent_record_offset":1508,"ancestor_record_offset":232,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":105,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":232,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":105,"record_offset":1624,"parent_record_offset":1560,"ancestor_record_offset":232,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":105,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":232,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":105,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":232,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":105,"record_offset":2020,"parent_record_offset":1452,"ancestor_record_offset":232,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":105,"record_offset":444,"parent_record_offset":420,"ancestor_record_offset":232,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":105,"record_offset":1452,"parent_record_offset":444,"ancestor_record_offset":232,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":105,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":232,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":105,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":232,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":105,"record_offset":1160,"parent_record_offset":1136,"ancestor_record_offset":232,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":105,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":232,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":105,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":232,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":105,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":232,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":105,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":232,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":105,"record_offset":2740,"parent_record_offset":2200,"ancestor_record_offset":232,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":105,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":232,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":105,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":232,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":105,"record_offset":1420,"parent_record_offset":472,"ancestor_record_offset":232,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":105,"record_offset":1688,"parent_record_offset":1508,"ancestor_record_offset":232,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":105,"record_offset":1792,"parent_record_offset":1768,"ancestor_record_offset":232,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":105,"record_offset":2464,"parent_record_offset":2440,"ancestor_record_offset":232,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":105,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":232,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":105,"record_offset":992,"parent_record_offset":968,"ancestor_record_offset":232,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":1560,"parent_record_offset":1532,"ancestor_record_offset":232,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":2416,"parent_record_offset":2268,"ancestor_record_offset":232,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":1596,"parent_record_offset":1560,"ancestor_record_offset":232,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":1768,"parent_record_offset":1736,"ancestor_record_offset":232,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":105,"record_offset":2440,"parent_record_offset":2416,"ancestor_record_offset":232,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":105,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":232,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":105,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":232,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":105,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":232,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":105,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":232,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":105,"record_offset":496,"parent_record_offset":472,"ancestor_record_offset":232,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":105,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":232,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":105,"record_offset":1112,"parent_record_offset":916,"ancestor_record_offset":232,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":1024,"parent_record_offset":940,"ancestor_record_offset":232,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":968,"parent_record_offset":940,"ancestor_record_offset":232,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":1368,"parent_record_offset":596,"ancestor_record_offset":232,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":940,"parent_record_offset":916,"ancestor_record_offset":232,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":232,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":548,"parent_record_offset":520,"ancestor_record_offset":232,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":596,"parent_record_offset":572,"ancestor_record_offset":232,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":105,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":232,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":105,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":232,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":105,"record_offset":884,"parent_record_offset":776,"ancestor_record_offset":232,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":105,"record_offset":776,"parent_record_offset":596,"ancestor_record_offset":232,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":105,"record_offset":916,"parent_record_offset":596,"ancestor_record_offset":232,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":105,"record_offset":636,"parent_record_offset":596,"ancestor_record_offset":232,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":105,"record_offset":720,"parent_record_offset":596,"ancestor_record_offset":232,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":105,"record_offset":1284,"parent_record_offset":596,"ancestor_record_offset":232,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":105,"record_offset":1736,"parent_record_offset":1712,"ancestor_record_offset":232,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00105.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::is_inside_map_bounds
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x686C
// 首条位置: {"module":1403,"inlinee":26732}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::world_space_to_screen_space
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 83, 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5C29
// 首条位置: {"module":411,"inlinee":23593}
// 编译实例 RVAs: 0x658130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":411,"record_offset":40484,"parent_record_offset":40448,"ancestor_record_offset":40448,"inlinee":23594,"source_line":1098,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":411,"record_offset":40892,"parent_record_offset":40448,"ancestor_record_offset":40448,"inlinee":15692,"source_line":1335,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":411,"record_offset":41468,"parent_record_offset":40448,"ancestor_record_offset":40448,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":411,"record_offset":41356,"parent_record_offset":40448,"ancestor_record_offset":40448,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::mul_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":411,"record_offset":41524,"parent_record_offset":40448,"ancestor_record_offset":40448,"inlinee":23596,"source_line":963,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":411,"record_offset":40684,"parent_record_offset":40600,"ancestor_record_offset":40448,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":411,"record_offset":40568,"parent_record_offset":40516,"ancestor_record_offset":40448,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":411,"record_offset":40540,"parent_record_offset":40516,"ancestor_record_offset":40448,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":411,"record_offset":40984,"parent_record_offset":40952,"ancestor_record_offset":40448,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":411,"record_offset":41492,"parent_record_offset":41468,"ancestor_record_offset":40448,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":411,"record_offset":41380,"parent_record_offset":41356,"ancestor_record_offset":40448,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$364::mul_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":411,"record_offset":41548,"parent_record_offset":41524,"ancestor_record_offset":40448,"inlinee":14660,"source_line":886,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::mul_vec4
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":411,"record_offset":40916,"parent_record_offset":40892,"ancestor_record_offset":40448,"inlinee":15693,"source_line":1157,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$10::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":411,"record_offset":40516,"parent_record_offset":40484,"ancestor_record_offset":40448,"inlinee":21866,"source_line":975,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$13::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":411,"record_offset":40832,"parent_record_offset":40484,"ancestor_record_offset":40448,"inlinee":15701,"source_line":998,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xxxx
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":411,"record_offset":40952,"parent_record_offset":40916,"ancestor_record_offset":40448,"inlinee":15695,"source_line":803,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::yyyy
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":411,"record_offset":41044,"parent_record_offset":40916,"ancestor_record_offset":40448,"inlinee":15696,"source_line":1313,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::zzzz
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":411,"record_offset":41168,"parent_record_offset":40916,"ancestor_record_offset":40448,"inlinee":15698,"source_line":1823,"archive":"_pdb/A/modules/00411.jsonl.gz"}
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
// 文件级候选接口: 31; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::event::EventReader<country_core::systems::backend::icc::IccProfileChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>", "bevy_ecs::event::EventReader<country_core::systems::backend::icc::IccProfileChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::backend::icc::IccProfileChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::utils::convert_function_expr_to_title::closure$0
// 信息 ID: 9031dafae7c19219fec3ef1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::convert_function_expr_to_title", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::drain_events<enum2$<country_core::resources::window::OsWindowCmd> >
// 信息 ID: 12a3a8471642a804c2d449db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["country_core::resources::window::OsWindowCmd"]; 字段布局 = unknown。
// [类型关系] country_core::utils::drain_events -> ["enum2$<country_core::resources::window::OsWindowCmd>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x2482fe -> 0x174dc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::get_last_event::closure$0
// 信息 ID: 3ce23f21b8ebb1edc43b4c35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::get_last_event", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::get_last_event<country_core::resources::mouse_capture::OsCursorVisibilityCmd>; module=105, inline_record=5488; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::get_last_event<enum2$<country_core::resources::mouse_capture::OsCursorCaptureCmd> >; module=105, inline_record=8156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::get_last_event<country_core::resources::mouse_capture::OsCursorVisibilityCmd>
// 信息 ID: 70d69ec0a884407dcf97643d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::get_last_event -> ["country_core::resources::mouse_capture::OsCursorVisibilityCmd"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x248547 -> 0x174f70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::get_last_event<enum2$<country_core::resources::mouse_capture::OsCursorCaptureCmd> >
// 信息 ID: d099949ef7ba92e8c88a3bd4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["country_core::resources::mouse_capture::OsCursorCaptureCmd"]; 字段布局 = unknown。
// [类型关系] country_core::utils::get_last_event -> ["enum2$<country_core::resources::mouse_capture::OsCursorCaptureCmd>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x2481fb -> 0x1751a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::get_res
// 信息 ID: 0ff5d1973e0c78035a20a1ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 111 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=19504; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=16620; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=18552; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=20648; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=22824; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=27616; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=29184; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=38628; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::get_res_mut
// 信息 ID: 2d830508dfa2e362a4a220cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=17484; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=22848; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=25448; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>; module=136, inline_record=1316; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster>; module=136, inline_record=4520; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster>; module=136, inline_record=7724; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::calc_dof; module=400, inline_record=51584; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass; module=415, inline_record=30204; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::is_inside_map_bounds
// 信息 ID: 56a28a7c0dcf5ccc353ca880
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::query_with_segment::closure_env$0>; module=1403, inline_record=17884; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::query_with_segment::closure_env$0>; module=1403, inline_record=17912; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::add_segment::closure_env$0>; module=1403, inline_record=25232; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::add_segment::closure_env$0>; module=1403, inline_record=25260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::world_space_to_screen_space
// 信息 ID: 0b84568e09e80f717de5b719
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x165c692 -> 0x658130 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x165c81e -> 0x658130 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x165c9f4 -> 0x658130 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::sheep::sheep_react_to_being_inside_buildings::sheep_reaction_emoji @ 0x1b3a658 -> 0x658130 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x1b5baca -> 0x658130 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x1b5bdea -> 0x658130 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid @ 0x1e666ed -> 0x658130 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid @ 0x1e6676a -> 0x658130 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::wall_outline::wall_outline::closure$0; module=411, inline_record=40448; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height; module=1302, inline_record=21908; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
