// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/render/pass_builder/pass_resources.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 71681489531897940dae84d600355bff5f63c28ce661a4fc96f8e5ee8ce2a216
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: pass_resources.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 412
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
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
// 函数符号: country_core::render::pass_builder::pass_resources::PassResources::new_from
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":412,"record_offset":252,"rva":5733440,"va":5374442560}
// 编译实例 RVAs: 0x577c40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t28533916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374442560,"raw_hex":"e89c64b301","size":5,"soft_fail":false,"target_va":5402978880,"va":5374444959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":412,"record_offset":416,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":12768,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":412,"record_offset":448,"parent_record_offset":416,"ancestor_record_offset":252,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":412,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":252,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":412,"record_offset":1952,"parent_record_offset":1920,"ancestor_record_offset":252,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":412,"record_offset":1892,"parent_record_offset":1856,"ancestor_record_offset":252,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":412,"record_offset":1828,"parent_record_offset":1428,"ancestor_record_offset":252,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":412,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":252,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":412,"record_offset":1468,"parent_record_offset":1428,"ancestor_record_offset":252,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":412,"record_offset":1920,"parent_record_offset":1856,"ancestor_record_offset":252,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":252,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":1492,"parent_record_offset":1468,"ancestor_record_offset":252,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":1608,"parent_record_offset":1544,"ancestor_record_offset":252,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":1696,"parent_record_offset":1672,"ancestor_record_offset":252,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::last_change_tick
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":412,"record_offset":2048,"parent_record_offset":2020,"ancestor_record_offset":252,"inlinee":9923,"source_line":2096,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":412,"record_offset":2076,"parent_record_offset":2020,"ancestor_record_offset":252,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":412,"record_offset":1428,"parent_record_offset":416,"ancestor_record_offset":252,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::last_change_tick
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":412,"record_offset":2020,"parent_record_offset":1428,"ancestor_record_offset":252,"inlinee":9922,"source_line":268,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":412,"record_offset":1160,"parent_record_offset":1136,"ancestor_record_offset":252,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":412,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":252,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":412,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":252,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":412,"record_offset":636,"parent_record_offset":612,"ancestor_record_offset":252,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":412,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":252,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":412,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":252,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":412,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":252,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":1632,"parent_record_offset":1608,"ancestor_record_offset":252,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":1396,"parent_record_offset":448,"ancestor_record_offset":252,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":1672,"parent_record_offset":1492,"ancestor_record_offset":252,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":412,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":252,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":412,"record_offset":1048,"parent_record_offset":1024,"ancestor_record_offset":252,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":412,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":252,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":412,"record_offset":1544,"parent_record_offset":1516,"ancestor_record_offset":252,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":412,"record_offset":1580,"parent_record_offset":1544,"ancestor_record_offset":252,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":412,"record_offset":1752,"parent_record_offset":1720,"ancestor_record_offset":252,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":412,"record_offset":2100,"parent_record_offset":2076,"ancestor_record_offset":252,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":412,"record_offset":2124,"parent_record_offset":2100,"ancestor_record_offset":252,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":412,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":252,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":412,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":252,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":412,"record_offset":472,"parent_record_offset":448,"ancestor_record_offset":252,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":412,"record_offset":496,"parent_record_offset":472,"ancestor_record_offset":252,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":412,"record_offset":1088,"parent_record_offset":892,"ancestor_record_offset":252,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":1000,"parent_record_offset":916,"ancestor_record_offset":252,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":944,"parent_record_offset":916,"ancestor_record_offset":252,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":1344,"parent_record_offset":572,"ancestor_record_offset":252,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":252,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":252,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":524,"parent_record_offset":496,"ancestor_record_offset":252,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":252,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":412,"record_offset":776,"parent_record_offset":752,"ancestor_record_offset":252,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":412,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":252,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":412,"record_offset":860,"parent_record_offset":752,"ancestor_record_offset":252,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":412,"record_offset":752,"parent_record_offset":572,"ancestor_record_offset":252,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":892,"parent_record_offset":572,"ancestor_record_offset":252,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":412,"record_offset":612,"parent_record_offset":572,"ancestor_record_offset":252,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":412,"record_offset":696,"parent_record_offset":572,"ancestor_record_offset":252,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":412,"record_offset":1260,"parent_record_offset":572,"ancestor_record_offset":252,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":412,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":252,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00412.jsonl.gz"}
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
// 完整结构及原始类型记录: pass_resources.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::pass_resources::PassResources
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 55
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::resource_info::ResourceInfo::calculate", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::graph::RenderGraph::compile<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::graph::RenderGraph::compile", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::recorder::RenderGraphCallback<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::recorder::RenderGraphCallback", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<rhapsody::rg::recorder::RenderGraphCallback<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "dyn$", "arguments": ["rhapsody::rg::recorder::RenderGraphCallback<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<rhapsody::rg::recorder::RenderGraphCallback<country_core::render::pass_builder::pass_resources::PassResources> >,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<rhapsody::rg::recorder::RenderGraphCallback<country_core::render::pass_builder::pass_resources::PassResources> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<dyn$<rhapsody::rg::recorder::RenderGraphCallback<country_core::render::pass_builder::pass_resources::PassResources> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<dyn$<rhapsody::rg::recorder::RenderGraphCallback<country_core::render::pass_builder::pass_resources::PassResources> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::compiled::CompiledRenderGraph::record_batch", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::compiled::CompiledRenderGraph::compute_batches<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::compiled::CompiledRenderGraph::compute_batches", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::compiled::CompiledRenderGraph::new<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::compiled::CompiledRenderGraph::new", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::compiled::CompiledRenderGraph::retire<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::compiled::CompiledRenderGraph::retire", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >", "constructor": "rhapsody::rg::temporal::impl$12::get_or_create_temporal", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "ref$<str$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String>", "constructor": "rhapsody::rg::temporal::impl$11::get_or_create_temporal", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "alloc::string::String"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >", "constructor": "rhapsody::rg::temporal::impl$11::get_or_create_temporal", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "ref$<str$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >", "constructor": "rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "ref$<str$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::compiled::impl$1::new::closure_env$0", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >", "rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["rhapsody::rg::resource::ResourceHandle", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String>", "constructor": "rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "alloc::string::String"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$0::with_context<rhapsody::raw::texture::Texture,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String> >", "constructor": "anyhow::context::impl$0::with_context", "arguments": ["rhapsody::raw::texture::Texture", "enum2$<rhapsody::raw::error::BackendError>", "alloc::string::String", "rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >", "constructor": "rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "ref$<str$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$0::with_context<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >", "constructor": "anyhow::context::impl$0::with_context", "arguments": ["rhapsody::raw::buffer::Buffer", "enum2$<rhapsody::raw::error::BackendError>", "alloc::string::String", "rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::recorder::RecordedPass", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::executor::GraphExecutor::execute", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<alloc::boxed::Box<dyn$<rhapsody::rg::recorder::RenderGraphCallback<country_core::render::pass_builder::pass_resources::PassResources> >,alloc::alloc::Global> >", "constructor": "core::option::Option", "arguments": ["alloc::boxed::Box<dyn$<rhapsody::rg::recorder::RenderGraphCallback<country_core::render::pass_builder::pass_resources::PassResources> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<alloc::boxed::Box<dyn$<rhapsody::rg::recorder::RenderGraphCallback<country_core::render::pass_builder::pass_resources::PassResources> >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["core::option::Option<alloc::boxed::Box<dyn$<rhapsody::rg::recorder::RenderGraphCallback<country_core::render::pass_builder::pass_resources::PassResources> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<alloc::boxed::Box<dyn$<rhapsody::rg::recorder::RenderGraphCallback<country_core::render::pass_builder::pass_resources::PassResources> >,alloc::alloc::Global> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<alloc::boxed::Box<dyn$<rhapsody::rg::recorder::RenderGraphCallback<country_core::render::pass_builder::pass_resources::PassResources> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::add_resource_access", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::recorder::impl$1::render<tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::recorder::impl$1::render", "arguments": ["tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0", "country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::pass_builder::PassBuilder", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::pass_builder::PassBuilder<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::pass_builder::PassBuilder<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::compiled::CompiledRenderGraph<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::compiled::CompiledRenderGraph", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::compiled::CompiledRenderGraph<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::compiled::CompiledRenderGraph<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::temporal::TemporalRenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::temporal::TemporalRenderGraphRecorder", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::temporal::TemporalRenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::temporal::TemporalRenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "core::option::Option", "arguments": ["rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >", "constructor": "enum2$", "arguments": ["core::option::Option<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::recorder::impl$1::render", "arguments": ["country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0", "country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::clear_rg_buffer::closure_env$0>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::clear_rg_buffer::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::clear_rg_color_image::closure_env$0>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::clear_rg_color_image::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32> >", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::recorder::impl$1::render<country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::recorder::impl$1::render", "arguments": ["country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0", "country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>,country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::recorder::impl$1::render", "arguments": ["country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>", "country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_color_image::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::recorder::impl$1::render", "arguments": ["country_core::systems::render_loop::clear_rg_color_image::closure_env$0", "country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_buffer::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::recorder::impl$1::render", "arguments": ["country_core::systems::render_loop::clear_rg_buffer::closure_env$0", "country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::recorder::impl$1::render<country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::recorder::impl$1::render", "arguments": ["country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0", "country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "ref_mut$", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "tuple$", "arguments": ["ref_mut$<rhapsody::rg::pass_api::RenderPassApi>", "ref_mut$<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$47::call_once", "arguments": ["tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >", "country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$47::call_once", "arguments": ["tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$47::call_once", "arguments": ["tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >", "country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$47::call_once", "arguments": ["tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: pass_resources.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: pass_resources.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::pass_resources::PassResources::new_from
// 信息 ID: e28ff7a8e8f244f4775b4b12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0x1a6ef2 -> 0x577c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<alloc::string::String> > @ 0x1a7713 -> 0x577c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x303fd9 -> 0x577c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::new_impl @ 0x660a84 -> 0x577c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketFlowers> @ 0xdcbcdc -> 0x577c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketWoodenPlank> @ 0xdcc5fc -> 0x577c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketBush> @ 0xdccf1c -> 0x577c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> > > @ 0xdcd812 -> 0x577c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0xdce142 -> 0x577c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof> @ 0xdcea5c -> 0x577c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::vermeer::GenerateStrokes> @ 0xdcf37c -> 0x577c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketRoofTile> @ 0xdcfc9c -> 0x577c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketMain> @ 0xdd05bc -> 0x577c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResDof> @ 0xdd0edc -> 0x577c40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
