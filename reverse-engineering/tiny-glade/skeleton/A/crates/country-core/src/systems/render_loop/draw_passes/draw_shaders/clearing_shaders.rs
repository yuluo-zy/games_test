// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/clearing_shaders.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b693e99cea86df3cf1297487ab527c745c279d2d84d4b2fe94d78a4844e36907
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: clearing_shaders.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 485
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"PDB_registry_path_only","version":"0.11.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::clearing_shaders::tree_trunk
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":485,"record_offset":22820,"rva":6897168,"va":5375606288}
// 编译实例 RVAs: 0x693e10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: clearing_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1390148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375606288,"raw_hex":"e8bcc9eaff","size":5,"soft_fail":false,"target_va":5374216800,"va":5375606943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1389748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375606288,"raw_hex":"e84ccbeaff","size":5,"soft_fail":false,"target_va":5374218416,"va":5375608159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406aa9b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t91177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375606288,"raw_hex":"e829640100","size":5,"soft_fail":false,"target_va":5375699376,"va":5375608194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: clearing_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t359176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375606288,"raw_hex":"e8087b0500","size":5,"soft_fail":false,"target_va":5375967552,"va":5375608371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6180
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: clearing_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7741168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375606288,"raw_hex":"e8f01e7600","size":5,"soft_fail":false,"target_va":5383348608,"va":5375607435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7741254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375606288,"raw_hex":"e8461f7600","size":5,"soft_fail":false,"target_va":5383349008,"va":5375607749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7742025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375606288,"raw_hex":"e849227600","size":5,"soft_fail":false,"target_va":5383349824,"va":5375607794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: clearing_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27986479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375606288,"raw_hex":"e82f0aab01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375606876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t27370628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375606288,"raw_hex":"e884a4a101","size":5,"soft_fail":false,"target_va":5402978880,"va":5375608247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7553710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375606288,"raw_hex":"e8ae427300","size":5,"soft_fail":false,"target_va":5383161424,"va":5375607709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":23012,"parent_record_offset":22820,"ancestor_record_offset":22820,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":485,"record_offset":26268,"parent_record_offset":22820,"ancestor_record_offset":22820,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance::get_dist
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/tree_edit.rs
//     evidence={"module":485,"record_offset":29668,"parent_record_offset":22820,"ancestor_record_offset":22820,"inlinee":26711,"source_line":85,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":485,"record_offset":23144,"parent_record_offset":22820,"ancestor_record_offset":22820,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":485,"record_offset":23064,"parent_record_offset":22820,"ancestor_record_offset":22820,"inlinee":21969,"source_line":613,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":485,"record_offset":23200,"parent_record_offset":23172,"ancestor_record_offset":22820,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":485,"record_offset":24632,"parent_record_offset":24600,"ancestor_record_offset":22820,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":24576,"parent_record_offset":24188,"ancestor_record_offset":22820,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":24600,"parent_record_offset":24576,"ancestor_record_offset":22820,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":24216,"parent_record_offset":24188,"ancestor_record_offset":22820,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":24264,"parent_record_offset":24240,"ancestor_record_offset":22820,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":24240,"parent_record_offset":24216,"ancestor_record_offset":22820,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":24356,"parent_record_offset":24292,"ancestor_record_offset":22820,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":24444,"parent_record_offset":24420,"ancestor_record_offset":22820,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":23172,"parent_record_offset":23144,"ancestor_record_offset":22820,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":24188,"parent_record_offset":23172,"ancestor_record_offset":22820,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":485,"record_offset":23920,"parent_record_offset":23896,"ancestor_record_offset":22820,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":23968,"parent_record_offset":23944,"ancestor_record_offset":22820,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":23896,"parent_record_offset":23872,"ancestor_record_offset":22820,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":485,"record_offset":23388,"parent_record_offset":23364,"ancestor_record_offset":22820,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":485,"record_offset":23472,"parent_record_offset":23448,"ancestor_record_offset":22820,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":485,"record_offset":23412,"parent_record_offset":23388,"ancestor_record_offset":22820,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":485,"record_offset":23584,"parent_record_offset":23560,"ancestor_record_offset":22820,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":485,"record_offset":29744,"parent_record_offset":29692,"ancestor_record_offset":22820,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":485,"record_offset":29800,"parent_record_offset":29692,"ancestor_record_offset":22820,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":485,"record_offset":29716,"parent_record_offset":29692,"ancestor_record_offset":22820,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":24380,"parent_record_offset":24356,"ancestor_record_offset":22820,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":24156,"parent_record_offset":23200,"ancestor_record_offset":22820,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":24420,"parent_record_offset":24240,"ancestor_record_offset":22820,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":24676,"parent_record_offset":23144,"ancestor_record_offset":22820,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":485,"record_offset":24524,"parent_record_offset":24500,"ancestor_record_offset":22820,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":485,"record_offset":23808,"parent_record_offset":23784,"ancestor_record_offset":22820,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":485,"record_offset":23728,"parent_record_offset":23704,"ancestor_record_offset":22820,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":485,"record_offset":24292,"parent_record_offset":24264,"ancestor_record_offset":22820,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":24328,"parent_record_offset":24292,"ancestor_record_offset":22820,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":24500,"parent_record_offset":24468,"ancestor_record_offset":22820,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":485,"record_offset":23944,"parent_record_offset":23920,"ancestor_record_offset":22820,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::frame_constants::impl$35::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/frame_constants.rs
//     evidence={"module":485,"record_offset":23036,"parent_record_offset":23012,"ancestor_record_offset":22820,"inlinee":27351,"source_line":54,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":485,"record_offset":23872,"parent_record_offset":23848,"ancestor_record_offset":22820,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":23224,"parent_record_offset":23200,"ancestor_record_offset":22820,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":23248,"parent_record_offset":23224,"ancestor_record_offset":22820,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":23848,"parent_record_offset":23652,"ancestor_record_offset":22820,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":23760,"parent_record_offset":23676,"ancestor_record_offset":22820,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":23704,"parent_record_offset":23676,"ancestor_record_offset":22820,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":24104,"parent_record_offset":23324,"ancestor_record_offset":22820,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":23676,"parent_record_offset":23652,"ancestor_record_offset":22820,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":23300,"parent_record_offset":23276,"ancestor_record_offset":22820,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":23276,"parent_record_offset":23248,"ancestor_record_offset":22820,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":23324,"parent_record_offset":23300,"ancestor_record_offset":22820,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":23532,"parent_record_offset":23504,"ancestor_record_offset":22820,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":23560,"parent_record_offset":23532,"ancestor_record_offset":22820,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":23620,"parent_record_offset":23504,"ancestor_record_offset":22820,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":23504,"parent_record_offset":23324,"ancestor_record_offset":22820,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":23652,"parent_record_offset":23324,"ancestor_record_offset":22820,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":23364,"parent_record_offset":23324,"ancestor_record_offset":22820,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":23448,"parent_record_offset":23324,"ancestor_record_offset":22820,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":24020,"parent_record_offset":23324,"ancestor_record_offset":22820,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":485,"record_offset":29692,"parent_record_offset":29668,"ancestor_record_offset":22820,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":485,"record_offset":23104,"parent_record_offset":23084,"ancestor_record_offset":22820,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":485,"record_offset":24468,"parent_record_offset":24444,"ancestor_record_offset":22820,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":485,"record_offset":23084,"parent_record_offset":23064,"ancestor_record_offset":22820,"inlinee":21970,"source_line":613,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::clearing_shaders::tree_canopy
// 记录形式: compiled_procedure; instances=1
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":485,"record_offset":29884,"rva":6900256,"va":5375609376}
// 编译实例 RVAs: 0x694a20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: clearing_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1393286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375609376,"raw_hex":"e87abdeaff","size":5,"soft_fail":false,"target_va":5374216800,"va":5375610081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406aa9b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t88464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375609376,"raw_hex":"e890590100","size":5,"soft_fail":false,"target_va":5375699376,"va":5375610907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: clearing_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t356471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375609376,"raw_hex":"e877700500","size":5,"soft_fail":false,"target_va":5375967552,"va":5375611076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6180
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: clearing_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7738116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375609376,"raw_hex":"e804137600","size":5,"soft_fail":false,"target_va":5383348608,"va":5375610487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7739245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375609376,"raw_hex":"e86d177600","size":5,"soft_fail":false,"target_va":5383349824,"va":5375610574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: clearing_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27983335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375609376,"raw_hex":"e8e7fdaa01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375610020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27367915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375609376,"raw_hex":"e8eb99a101","size":5,"soft_fail":false,"target_va":5402978880,"va":5375610960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":30088,"parent_record_offset":29884,"ancestor_record_offset":29884,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":485,"record_offset":31868,"parent_record_offset":29884,"ancestor_record_offset":29884,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::to_radians
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":485,"record_offset":31840,"parent_record_offset":29884,"ancestor_record_offset":29884,"inlinee":22232,"source_line":903,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":485,"record_offset":33456,"parent_record_offset":29884,"ancestor_record_offset":29884,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance::get_dist
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/tree_edit.rs
//     evidence={"module":485,"record_offset":35244,"parent_record_offset":29884,"ancestor_record_offset":29884,"inlinee":26711,"source_line":85,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::MainCamera::lens
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":485,"record_offset":31784,"parent_record_offset":29884,"ancestor_record_offset":29884,"inlinee":22309,"source_line":441,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":485,"record_offset":30220,"parent_record_offset":29884,"ancestor_record_offset":29884,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":485,"record_offset":30140,"parent_record_offset":29884,"ancestor_record_offset":29884,"inlinee":21969,"source_line":613,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":485,"record_offset":30276,"parent_record_offset":30248,"ancestor_record_offset":29884,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":485,"record_offset":31708,"parent_record_offset":31676,"ancestor_record_offset":29884,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":31652,"parent_record_offset":31264,"ancestor_record_offset":29884,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":31676,"parent_record_offset":31652,"ancestor_record_offset":29884,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":31292,"parent_record_offset":31264,"ancestor_record_offset":29884,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":31340,"parent_record_offset":31316,"ancestor_record_offset":29884,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":31316,"parent_record_offset":31292,"ancestor_record_offset":29884,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":31432,"parent_record_offset":31368,"ancestor_record_offset":29884,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":31520,"parent_record_offset":31496,"ancestor_record_offset":29884,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":30248,"parent_record_offset":30220,"ancestor_record_offset":29884,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":31264,"parent_record_offset":30248,"ancestor_record_offset":29884,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":485,"record_offset":30996,"parent_record_offset":30972,"ancestor_record_offset":29884,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":31044,"parent_record_offset":31020,"ancestor_record_offset":29884,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":30972,"parent_record_offset":30948,"ancestor_record_offset":29884,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":485,"record_offset":30464,"parent_record_offset":30440,"ancestor_record_offset":29884,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":485,"record_offset":30548,"parent_record_offset":30524,"ancestor_record_offset":29884,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":485,"record_offset":30488,"parent_record_offset":30464,"ancestor_record_offset":29884,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":485,"record_offset":30660,"parent_record_offset":30636,"ancestor_record_offset":29884,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":485,"record_offset":35320,"parent_record_offset":35268,"ancestor_record_offset":29884,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":485,"record_offset":35376,"parent_record_offset":35268,"ancestor_record_offset":29884,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":485,"record_offset":35292,"parent_record_offset":35268,"ancestor_record_offset":29884,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":31456,"parent_record_offset":31432,"ancestor_record_offset":29884,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":31232,"parent_record_offset":30276,"ancestor_record_offset":29884,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":31496,"parent_record_offset":31316,"ancestor_record_offset":29884,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":31752,"parent_record_offset":30220,"ancestor_record_offset":29884,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":485,"record_offset":31600,"parent_record_offset":31576,"ancestor_record_offset":29884,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":485,"record_offset":30884,"parent_record_offset":30860,"ancestor_record_offset":29884,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":485,"record_offset":30804,"parent_record_offset":30780,"ancestor_record_offset":29884,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":485,"record_offset":31368,"parent_record_offset":31340,"ancestor_record_offset":29884,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":31404,"parent_record_offset":31368,"ancestor_record_offset":29884,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":31576,"parent_record_offset":31544,"ancestor_record_offset":29884,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":485,"record_offset":31020,"parent_record_offset":30996,"ancestor_record_offset":29884,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::frame_constants::impl$35::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/frame_constants.rs
//     evidence={"module":485,"record_offset":30112,"parent_record_offset":30088,"ancestor_record_offset":29884,"inlinee":27351,"source_line":54,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::CameraModeStates::current_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":485,"record_offset":31808,"parent_record_offset":31784,"ancestor_record_offset":29884,"inlinee":22039,"source_line":149,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":485,"record_offset":30948,"parent_record_offset":30924,"ancestor_record_offset":29884,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":30300,"parent_record_offset":30276,"ancestor_record_offset":29884,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":30324,"parent_record_offset":30300,"ancestor_record_offset":29884,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":30924,"parent_record_offset":30728,"ancestor_record_offset":29884,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":30836,"parent_record_offset":30752,"ancestor_record_offset":29884,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":30780,"parent_record_offset":30752,"ancestor_record_offset":29884,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":31180,"parent_record_offset":30400,"ancestor_record_offset":29884,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":30752,"parent_record_offset":30728,"ancestor_record_offset":29884,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":30376,"parent_record_offset":30352,"ancestor_record_offset":29884,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":30352,"parent_record_offset":30324,"ancestor_record_offset":29884,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":30400,"parent_record_offset":30376,"ancestor_record_offset":29884,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":30608,"parent_record_offset":30580,"ancestor_record_offset":29884,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":30636,"parent_record_offset":30608,"ancestor_record_offset":29884,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":30696,"parent_record_offset":30580,"ancestor_record_offset":29884,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":30580,"parent_record_offset":30400,"ancestor_record_offset":29884,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":30728,"parent_record_offset":30400,"ancestor_record_offset":29884,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":30440,"parent_record_offset":30400,"ancestor_record_offset":29884,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":30524,"parent_record_offset":30400,"ancestor_record_offset":29884,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":31096,"parent_record_offset":30400,"ancestor_record_offset":29884,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":485,"record_offset":35268,"parent_record_offset":35244,"ancestor_record_offset":29884,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":485,"record_offset":30180,"parent_record_offset":30160,"ancestor_record_offset":29884,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":485,"record_offset":31544,"parent_record_offset":31520,"ancestor_record_offset":29884,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":485,"record_offset":30160,"parent_record_offset":30140,"ancestor_record_offset":29884,"inlinee":21970,"source_line":613,"archive":"_pdb/A/modules/00485.jsonl.gz"}
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
// 完整结构及原始类型记录: clearing_shaders.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: clearing_shaders.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/clearing_shaders.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: clearing_shaders.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::clearing_shaders::tree_trunk
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":485,"record_offset":26268,"parent_record_offset":22820,"ancestor_record_offset":22820,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00485.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::clearing_shaders::tree_canopy
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":485,"record_offset":33456,"parent_record_offset":29884,"ancestor_record_offset":29884,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00485.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: clearing_shaders.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::clearing_shaders::tree_canopy
// 信息 ID: 9078ddf3b50b6f02b7377012
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::clearing_shaders::tree_trunk
// 信息 ID: 0d4ad3af0711c95e8b9f60bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
