// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5b2c39448946d7469d854b3ab7c0d517d9c75a6ce5e1a801b339161befdce58a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: common_draw_params.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 388, 394, 441, 485, 490
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
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
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":441,"record_offset":12888,"rva":6209856,"va":5374918976}
// 编译实例 RVAs: 0x5ec140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28059562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374918976,"raw_hex":"e8aa27ac01","size":5,"soft_fail":false,"target_va":5402978880,"va":5374919313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":441,"record_offset":14624,"parent_record_offset":12888,"ancestor_record_offset":12888,"inlinee":12632,"source_line":855,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":441,"record_offset":13068,"parent_record_offset":12888,"ancestor_record_offset":12888,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":441,"record_offset":13124,"parent_record_offset":13096,"ancestor_record_offset":12888,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":441,"record_offset":14548,"parent_record_offset":14516,"ancestor_record_offset":12888,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":14492,"parent_record_offset":14104,"ancestor_record_offset":12888,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":14516,"parent_record_offset":14492,"ancestor_record_offset":12888,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":14132,"parent_record_offset":14104,"ancestor_record_offset":12888,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":14180,"parent_record_offset":14156,"ancestor_record_offset":12888,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":14156,"parent_record_offset":14132,"ancestor_record_offset":12888,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":14272,"parent_record_offset":14208,"ancestor_record_offset":12888,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":14360,"parent_record_offset":14336,"ancestor_record_offset":12888,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":13096,"parent_record_offset":13068,"ancestor_record_offset":12888,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":14104,"parent_record_offset":13096,"ancestor_record_offset":12888,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":441,"record_offset":13836,"parent_record_offset":13812,"ancestor_record_offset":12888,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":13884,"parent_record_offset":13860,"ancestor_record_offset":12888,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":13812,"parent_record_offset":13788,"ancestor_record_offset":12888,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":441,"record_offset":13312,"parent_record_offset":13288,"ancestor_record_offset":12888,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":441,"record_offset":13396,"parent_record_offset":13372,"ancestor_record_offset":12888,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":441,"record_offset":13336,"parent_record_offset":13312,"ancestor_record_offset":12888,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":441,"record_offset":13500,"parent_record_offset":13476,"ancestor_record_offset":12888,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":14296,"parent_record_offset":14272,"ancestor_record_offset":12888,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":14072,"parent_record_offset":13124,"ancestor_record_offset":12888,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":14336,"parent_record_offset":14156,"ancestor_record_offset":12888,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":14592,"parent_record_offset":13068,"ancestor_record_offset":12888,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":441,"record_offset":14440,"parent_record_offset":14416,"ancestor_record_offset":12888,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":441,"record_offset":13724,"parent_record_offset":13700,"ancestor_record_offset":12888,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":441,"record_offset":13644,"parent_record_offset":13620,"ancestor_record_offset":12888,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":441,"record_offset":14208,"parent_record_offset":14180,"ancestor_record_offset":12888,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":14244,"parent_record_offset":14208,"ancestor_record_offset":12888,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":14416,"parent_record_offset":14384,"ancestor_record_offset":12888,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":441,"record_offset":13860,"parent_record_offset":13836,"ancestor_record_offset":12888,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":441,"record_offset":13788,"parent_record_offset":13764,"ancestor_record_offset":12888,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":13148,"parent_record_offset":13124,"ancestor_record_offset":12888,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":13172,"parent_record_offset":13148,"ancestor_record_offset":12888,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":13764,"parent_record_offset":13568,"ancestor_record_offset":12888,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":13676,"parent_record_offset":13592,"ancestor_record_offset":12888,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":13620,"parent_record_offset":13592,"ancestor_record_offset":12888,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":14020,"parent_record_offset":13248,"ancestor_record_offset":12888,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":13592,"parent_record_offset":13568,"ancestor_record_offset":12888,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":13224,"parent_record_offset":13200,"ancestor_record_offset":12888,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":13200,"parent_record_offset":13172,"ancestor_record_offset":12888,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":13248,"parent_record_offset":13224,"ancestor_record_offset":12888,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":13452,"parent_record_offset":13428,"ancestor_record_offset":12888,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":13476,"parent_record_offset":13452,"ancestor_record_offset":12888,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":13536,"parent_record_offset":13428,"ancestor_record_offset":12888,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":13428,"parent_record_offset":13248,"ancestor_record_offset":12888,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":13568,"parent_record_offset":13248,"ancestor_record_offset":12888,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":13288,"parent_record_offset":13248,"ancestor_record_offset":12888,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":13372,"parent_record_offset":13248,"ancestor_record_offset":12888,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":13936,"parent_record_offset":13248,"ancestor_record_offset":12888,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":441,"record_offset":14384,"parent_record_offset":14360,"ancestor_record_offset":12888,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":490,"record_offset":51508,"rva":7005248,"va":5375714368}
// 编译实例 RVAs: 0x6ae440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1496187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375714368,"raw_hex":"e8852be9ff","size":5,"soft_fail":false,"target_va":5374218416,"va":5375714598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t252266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375714368,"raw_hex":"e86ad90300","size":5,"soft_fail":false,"target_va":5375967552,"va":5375715281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7628924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375714368,"raw_hex":"e87c687400","size":5,"soft_fail":false,"target_va":5383343584,"va":5375714655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df5000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7629014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375714368,"raw_hex":"e8d6687400","size":5,"soft_fail":false,"target_va":5383344128,"va":5375715109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df64b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7634384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375714368,"raw_hex":"e8d07d7400","size":5,"soft_fail":false,"target_va":5383349424,"va":5375715035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7635272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375714368,"raw_hex":"e848817400","size":5,"soft_fail":false,"target_va":5383349824,"va":5375714547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7635521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375714368,"raw_hex":"e841827400","size":5,"soft_fail":false,"target_va":5383350624,"va":5375715098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7635999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375714368,"raw_hex":"e81f847400","size":5,"soft_fail":false,"target_va":5383351008,"va":5375715004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7636352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375714368,"raw_hex":"e880857400","size":5,"soft_fail":false,"target_va":5383351424,"va":5375715067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27878839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375714368,"raw_hex":"e8b765a901","size":5,"soft_fail":false,"target_va":5403593360,"va":5375714516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_dummy_shadowmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":490,"record_offset":51672,"parent_record_offset":51508,"ancestor_record_offset":51508,"inlinee":27548,"source_line":230,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_matrix
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":490,"record_offset":51752,"parent_record_offset":51508,"ancestor_record_offset":51508,"inlinee":27549,"source_line":23,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":490,"record_offset":51908,"parent_record_offset":51828,"ancestor_record_offset":51508,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":490,"record_offset":52180,"parent_record_offset":52128,"ancestor_record_offset":51508,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":490,"record_offset":52152,"parent_record_offset":52128,"ancestor_record_offset":51508,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":490,"record_offset":51700,"parent_record_offset":51672,"ancestor_record_offset":51508,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":490,"record_offset":52444,"parent_record_offset":51776,"ancestor_record_offset":51508,"inlinee":22357,"source_line":103,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":490,"record_offset":51776,"parent_record_offset":51752,"ancestor_record_offset":51508,"inlinee":27550,"source_line":215,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::quat_to_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":490,"record_offset":51828,"parent_record_offset":51776,"ancestor_record_offset":51508,"inlinee":27551,"source_line":181,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::impl$20::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":490,"record_offset":51932,"parent_record_offset":51908,"ancestor_record_offset":51508,"inlinee":27553,"source_line":963,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":490,"record_offset":52020,"parent_record_offset":51828,"ancestor_record_offset":51508,"inlinee":14703,"source_line":92,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$10::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":490,"record_offset":52128,"parent_record_offset":51776,"ancestor_record_offset":51508,"inlinee":21866,"source_line":975,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$44::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":490,"record_offset":51980,"parent_record_offset":51956,"ancestor_record_offset":51508,"inlinee":27555,"source_line":1273,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$47::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":490,"record_offset":52380,"parent_record_offset":51776,"ancestor_record_offset":51508,"inlinee":27557,"source_line":1300,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_wall_color_customization
// 记录形式: compiled_procedure; instances=1
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":490,"record_offset":52488,"rva":7006240,"va":5375715360}
// 编译实例 RVAs: 0x6ae820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1498688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375715360,"raw_hex":"e8c021e9ff","size":5,"soft_fail":false,"target_va":5374216800,"va":5375715483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t251548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375715360,"raw_hex":"e89cd60300","size":5,"soft_fail":false,"target_va":5375967552,"va":5375715999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfabb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7651688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375715360,"raw_hex":"e868c17400","size":5,"soft_fail":false,"target_va":5383367600,"va":5375715907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27877939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375715360,"raw_hex":"e83362a901","size":5,"soft_fail":false,"target_va":5403593360,"va":5375715416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":490,"record_offset":52664,"parent_record_offset":52488,"ancestor_record_offset":52488,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_roof_color_customization
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55D5
// 首条位置: {"module":388,"inlinee":21973}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":388,"record_offset":24356,"parent_record_offset":24328,"ancestor_record_offset":24328,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_window_color_customization_textures
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x580C
// 首条位置: {"module":394,"inlinee":22540}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":394,"record_offset":8932,"parent_record_offset":8904,"ancestor_record_offset":8904,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00394.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_brick_erase_highlight
// 记录形式: compiled_procedure; instances=1
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":490,"record_offset":52752,"rva":7006912,"va":5375716032}
// 编译实例 RVAs: 0x6aeac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t250132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375716032,"raw_hex":"e814d10300","size":5,"soft_fail":false,"target_va":5375967552,"va":5375717415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7631390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375716032,"raw_hex":"e81e727400","size":5,"soft_fail":false,"target_va":5383348608,"va":5375717213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7631825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375716032,"raw_hex":"e8d1737400","size":5,"soft_fail":false,"target_va":5383349008,"va":5375717178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7632681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375716032,"raw_hex":"e829777400","size":5,"soft_fail":false,"target_va":5383349824,"va":5375717138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27876260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375716032,"raw_hex":"e8a45ba901","size":5,"soft_fail":false,"target_va":5403593360,"va":5375717095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t27261613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375716032,"raw_hex":"e8adfa9f01","size":5,"soft_fail":false,"target_va":5402978880,"va":5375717262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":490,"record_offset":56056,"parent_record_offset":52752,"ancestor_record_offset":52752,"inlinee":22044,"source_line":28,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":490,"record_offset":52932,"parent_record_offset":52752,"ancestor_record_offset":52752,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":490,"record_offset":52988,"parent_record_offset":52960,"ancestor_record_offset":52752,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":490,"record_offset":54420,"parent_record_offset":54388,"ancestor_record_offset":52752,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":54364,"parent_record_offset":53976,"ancestor_record_offset":52752,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":54388,"parent_record_offset":54364,"ancestor_record_offset":52752,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":54004,"parent_record_offset":53976,"ancestor_record_offset":52752,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":54052,"parent_record_offset":54028,"ancestor_record_offset":52752,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":54028,"parent_record_offset":54004,"ancestor_record_offset":52752,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":54144,"parent_record_offset":54080,"ancestor_record_offset":52752,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":54232,"parent_record_offset":54208,"ancestor_record_offset":52752,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":490,"record_offset":52960,"parent_record_offset":52932,"ancestor_record_offset":52752,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":490,"record_offset":53976,"parent_record_offset":52960,"ancestor_record_offset":52752,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":490,"record_offset":53708,"parent_record_offset":53684,"ancestor_record_offset":52752,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":490,"record_offset":53756,"parent_record_offset":53732,"ancestor_record_offset":52752,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":490,"record_offset":53684,"parent_record_offset":53660,"ancestor_record_offset":52752,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":490,"record_offset":53176,"parent_record_offset":53152,"ancestor_record_offset":52752,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":490,"record_offset":53260,"parent_record_offset":53236,"ancestor_record_offset":52752,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":490,"record_offset":53200,"parent_record_offset":53176,"ancestor_record_offset":52752,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":490,"record_offset":53372,"parent_record_offset":53348,"ancestor_record_offset":52752,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":54168,"parent_record_offset":54144,"ancestor_record_offset":52752,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":53944,"parent_record_offset":52988,"ancestor_record_offset":52752,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":54208,"parent_record_offset":54028,"ancestor_record_offset":52752,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":54464,"parent_record_offset":52932,"ancestor_record_offset":52752,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":56080,"parent_record_offset":56056,"ancestor_record_offset":52752,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":490,"record_offset":54312,"parent_record_offset":54288,"ancestor_record_offset":52752,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":490,"record_offset":53596,"parent_record_offset":53572,"ancestor_record_offset":52752,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":490,"record_offset":53516,"parent_record_offset":53492,"ancestor_record_offset":52752,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":490,"record_offset":54080,"parent_record_offset":54052,"ancestor_record_offset":52752,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":490,"record_offset":54116,"parent_record_offset":54080,"ancestor_record_offset":52752,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":490,"record_offset":54288,"parent_record_offset":54256,"ancestor_record_offset":52752,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":490,"record_offset":53732,"parent_record_offset":53708,"ancestor_record_offset":52752,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":490,"record_offset":53660,"parent_record_offset":53636,"ancestor_record_offset":52752,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":490,"record_offset":53012,"parent_record_offset":52988,"ancestor_record_offset":52752,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":490,"record_offset":53036,"parent_record_offset":53012,"ancestor_record_offset":52752,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":490,"record_offset":53636,"parent_record_offset":53440,"ancestor_record_offset":52752,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":53548,"parent_record_offset":53464,"ancestor_record_offset":52752,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":53492,"parent_record_offset":53464,"ancestor_record_offset":52752,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":53892,"parent_record_offset":53112,"ancestor_record_offset":52752,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":53464,"parent_record_offset":53440,"ancestor_record_offset":52752,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":53088,"parent_record_offset":53064,"ancestor_record_offset":52752,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":53064,"parent_record_offset":53036,"ancestor_record_offset":52752,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":53112,"parent_record_offset":53088,"ancestor_record_offset":52752,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":53320,"parent_record_offset":53292,"ancestor_record_offset":52752,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":53348,"parent_record_offset":53320,"ancestor_record_offset":52752,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":53408,"parent_record_offset":53292,"ancestor_record_offset":52752,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":53292,"parent_record_offset":53112,"ancestor_record_offset":52752,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":53440,"parent_record_offset":53112,"ancestor_record_offset":52752,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":490,"record_offset":53152,"parent_record_offset":53112,"ancestor_record_offset":52752,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":490,"record_offset":53236,"parent_record_offset":53112,"ancestor_record_offset":52752,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":490,"record_offset":53808,"parent_record_offset":53112,"ancestor_record_offset":52752,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":490,"record_offset":54256,"parent_record_offset":54232,"ancestor_record_offset":52752,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00490.jsonl.gz"}
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
// 完整结构及原始类型记录: common_draw_params.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: common_draw_params.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: common_draw_params.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":490,"record_offset":51700,"parent_record_offset":51672,"ancestor_record_offset":51508,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00490.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_wall_color_customization
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":490,"record_offset":52664,"parent_record_offset":52488,"ancestor_record_offset":52488,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00490.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_roof_color_customization
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":388,"record_offset":24356,"parent_record_offset":24328,"ancestor_record_offset":24328,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00388.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_window_color_customization_textures
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":394,"record_offset":8932,"parent_record_offset":8904,"ancestor_record_offset":8904,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00394.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":5288464,"caller_aliases":{"alias_group_rva":5288464,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::main_pass::main_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5289355,"position_among_mapped_render_targets":1,"mapped_render_target_count":3,"previous_callsite":null,"next_callsite":5290053,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5288464,"caller_aliases":{"alias_group_rva":5288464,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::main_pass::main_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5290053,"position_among_mapped_render_targets":2,"mapped_render_target_count":3,"previous_callsite":5289355,"next_callsite":5290739,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5288464,"caller_aliases":{"alias_group_rva":5288464,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::main_pass::main_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5290739,"position_among_mapped_render_targets":3,"mapped_render_target_count":3,"previous_callsite":5290053,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5292000,"caller_aliases":{"alias_group_rva":5292000,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5292891,"position_among_mapped_render_targets":1,"mapped_render_target_count":3,"previous_callsite":null,"next_callsite":5293589,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5292000,"caller_aliases":{"alias_group_rva":5292000,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5293589,"position_among_mapped_render_targets":2,"mapped_render_target_count":3,"previous_callsite":5292891,"next_callsite":5294275,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5292000,"caller_aliases":{"alias_group_rva":5292000,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5294275,"position_among_mapped_render_targets":3,"mapped_render_target_count":3,"previous_callsite":5293589,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5295536,"caller_aliases":{"alias_group_rva":5295536,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5295787,"position_among_mapped_render_targets":1,"mapped_render_target_count":2,"previous_callsite":null,"next_callsite":5296492,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5295536,"caller_aliases":{"alias_group_rva":5295536,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5296492,"position_among_mapped_render_targets":2,"mapped_render_target_count":2,"previous_callsite":5295787,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":6324144,"caller_aliases":{"alias_group_rva":6324144,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6325321,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":6326144,"caller_aliases":{"alias_group_rva":6326144,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6326357,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":6327216,"caller_aliases":{"alias_group_rva":6327216,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6327921,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":6331248,"caller_aliases":{"alias_group_rva":6331248,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6332902,"position_among_mapped_render_targets":3,"mapped_render_target_count":3,"previous_callsite":6332086,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/mod.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":6343056,"caller_aliases":{"alias_group_rva":6343056,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::transparency_pass::pre_taa_transparency_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6343255,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":6344448,"caller_aliases":{"alias_group_rva":6344448,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6344637,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":6955616,"caller_aliases":{"alias_group_rva":6955616,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::ui_pass::ui_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6955786,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":6956368,"caller_aliases":{"alias_group_rva":6956368,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6956538,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":6957120,"caller_aliases":{"alias_group_rva":6957120,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::wireframe_pass::wireframe_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6957290,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: common_draw_params.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>,bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>", "bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_brick_erase_highlight
// 信息 ID: d35beb40bfe06404c2c0b074
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$14,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x52be26 -> 0x6aeac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<0> @ 0x6958c5 -> 0x6aeac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<1> @ 0x696132 -> 0x6aeac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common
// 信息 ID: fa86d51ef493c05a1d976f18
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x50b71a -> 0x6ae440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x50b9dc -> 0x6ae440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x50bc89 -> 0x6ae440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x50c4ea -> 0x6ae440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x50c7ac -> 0x6ae440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x50ca59 -> 0x6ae440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x50d02a -> 0x6ae440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x50d2eb -> 0x6ae440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass @ 0x6085a4 -> 0x6ae440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass @ 0x6089d7 -> 0x6ae440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass @ 0x608ff3 -> 0x6ae440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x60a373 -> 0x6ae440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::pre_taa_transparency_pass @ 0x60cbd7 -> 0x6ae440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass @ 0x60d130 -> 0x6ae440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::ui_pass @ 0x6a2470 -> 0x6ae440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass @ 0x6a2760 -> 0x6ae440 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_roof_color_customization
// 信息 ID: 72668b70b53ccfc8cbcee21d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_shader_nani; module=388, inline_record=24328; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$16,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=46788; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$13,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=70004; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$14,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=123356; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<0>; module=485, inline_record=41024; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<1>; module=485, inline_record=46852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_wall_color_customization
// 信息 ID: 0ea6a88d3b8aea53af6db2ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$16,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x5255d9 -> 0x6ae820 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$32,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x5271b8 -> 0x6ae820 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$13,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x5276ab -> 0x6ae820 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$14,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x52bdab -> 0x6ae820 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<0> @ 0x6958dc -> 0x6ae820 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<1> @ 0x696149 -> 0x6ae820 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_window_color_customization_textures
// 信息 ID: d01fc74463982c0107fe34cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$15,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=8904; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$30,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=80360; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$29,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=118592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
// 信息 ID: 1d3f725c700d8e295b354696
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x50b58b -> 0x5ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x50b845 -> 0x5ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x50baf3 -> 0x5ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x50c35b -> 0x5ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x50c615 -> 0x5ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x50c8c3 -> 0x5ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x50ceab -> 0x5ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x50d16c -> 0x5ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass @ 0x608449 -> 0x5ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass @ 0x608855 -> 0x5ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass @ 0x608e71 -> 0x5ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x60a1e6 -> 0x5ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::pre_taa_transparency_pass @ 0x60ca57 -> 0x5ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass @ 0x60cfbd -> 0x5ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::ui_pass @ 0x6a230a -> 0x5ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass @ 0x6a25fa -> 0x5ec140 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
