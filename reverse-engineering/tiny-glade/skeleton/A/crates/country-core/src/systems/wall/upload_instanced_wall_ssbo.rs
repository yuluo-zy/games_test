// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/upload_instanced_wall_ssbo.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: cdc4dd6c9fb955c6cfbbd8831805d1be7d717066f3c3902223d7fcfbdd89b924
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: upload_instanced_wall_ssbo.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 463
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::upload_instanced_wall_ssbo::gpu_flush_brick_buffers
// 记录形式: compiled_procedure; instances=1
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":463,"record_offset":23600,"rva":6562448,"va":5375271568}
// 编译实例 RVAs: 0x642290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140d32690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_instanced_wall_ssbo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7275479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375271568,"raw_hex":"e8d7036f00","size":5,"soft_fail":false,"target_va":5382547088,"va":5375271604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27707014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375271568,"raw_hex":"e886c6a601","size":5,"soft_fail":false,"target_va":5402978880,"va":5375271861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::InstancedWall::upload_to_storage_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-278212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375271568,"raw_hex":"e83cc1fbff","size":5,"soft_fail":false,"target_va":5374993552,"va":5375271759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140f5b150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_instanced_wall_ssbo.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t9538978","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375271568,"raw_hex":"e9a28d9100","size":5,"soft_fail":false,"target_va":5384810832,"va":5375271849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":463,"record_offset":23784,"parent_record_offset":23600,"ancestor_record_offset":23600,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":463,"record_offset":24768,"parent_record_offset":23600,"ancestor_record_offset":23600,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":463,"record_offset":24072,"parent_record_offset":23600,"ancestor_record_offset":23600,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":463,"record_offset":23876,"parent_record_offset":23600,"ancestor_record_offset":23600,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":463,"record_offset":23840,"parent_record_offset":23600,"ancestor_record_offset":23600,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":463,"record_offset":24824,"parent_record_offset":23600,"ancestor_record_offset":23600,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":463,"record_offset":24848,"parent_record_offset":24824,"ancestor_record_offset":23600,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":463,"record_offset":23808,"parent_record_offset":23784,"ancestor_record_offset":23600,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":463,"record_offset":24792,"parent_record_offset":24768,"ancestor_record_offset":23600,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":463,"record_offset":24432,"parent_record_offset":24100,"ancestor_record_offset":23600,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":463,"record_offset":24148,"parent_record_offset":24100,"ancestor_record_offset":23600,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":463,"record_offset":23924,"parent_record_offset":23900,"ancestor_record_offset":23600,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":463,"record_offset":23948,"parent_record_offset":23924,"ancestor_record_offset":23600,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":463,"record_offset":24100,"parent_record_offset":24072,"ancestor_record_offset":23600,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":463,"record_offset":23900,"parent_record_offset":23876,"ancestor_record_offset":23600,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":463,"record_offset":24396,"parent_record_offset":24372,"ancestor_record_offset":23600,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":463,"record_offset":24200,"parent_record_offset":24176,"ancestor_record_offset":23600,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":463,"record_offset":24248,"parent_record_offset":24224,"ancestor_record_offset":23600,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":463,"record_offset":24372,"parent_record_offset":24148,"ancestor_record_offset":23600,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":463,"record_offset":24732,"parent_record_offset":24100,"ancestor_record_offset":23600,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":463,"record_offset":24176,"parent_record_offset":24148,"ancestor_record_offset":23600,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":463,"record_offset":24456,"parent_record_offset":24432,"ancestor_record_offset":23600,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":463,"record_offset":24648,"parent_record_offset":24100,"ancestor_record_offset":23600,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":463,"record_offset":24224,"parent_record_offset":24200,"ancestor_record_offset":23600,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":463,"record_offset":24320,"parent_record_offset":24296,"ancestor_record_offset":23600,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":463,"record_offset":24020,"parent_record_offset":23996,"ancestor_record_offset":23600,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":463,"record_offset":24612,"parent_record_offset":24588,"ancestor_record_offset":23600,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":463,"record_offset":24560,"parent_record_offset":24524,"ancestor_record_offset":23600,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":463,"record_offset":24672,"parent_record_offset":24648,"ancestor_record_offset":23600,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":463,"record_offset":24272,"parent_record_offset":24248,"ancestor_record_offset":23600,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":463,"record_offset":23972,"parent_record_offset":23948,"ancestor_record_offset":23600,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":463,"record_offset":24696,"parent_record_offset":24672,"ancestor_record_offset":23600,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":463,"record_offset":24296,"parent_record_offset":24272,"ancestor_record_offset":23600,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":463,"record_offset":23996,"parent_record_offset":23972,"ancestor_record_offset":23600,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":463,"record_offset":24588,"parent_record_offset":24524,"ancestor_record_offset":23600,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":463,"record_offset":24524,"parent_record_offset":24100,"ancestor_record_offset":23600,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00463.jsonl.gz"}
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
// 完整结构及原始类型记录: upload_instanced_wall_ssbo.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: upload_instanced_wall_ssbo.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: upload_instanced_wall_ssbo.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::query::Query<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,tuple$<> >

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::upload_instanced_wall_ssbo::gpu_flush_brick_buffers
// 信息 ID: 01ceadcb441130aafa8f5456
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 69412f9325be3b97d7834edd: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,tuple$<> >),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,tuple$<> >)> @ 0x57a64 -> 0x642290 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,tuple$<> >),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,tuple$<> >)> > @ 0xc453e -> 0x642290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
