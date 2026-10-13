// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/utils/child_wall.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 8835f64b1f573e8166e7c4d6d83254809cbdce0d7bfe3343804fb0203ecfd05f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: child_wall.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 540, 553, 566
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::child_wall::ChildWall::new
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79A7
// 首条位置: {"module":553,"inlinee":31143}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":553,"record_offset":71896,"parent_record_offset":71868,"ancestor_record_offset":71868,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":553,"record_offset":71952,"parent_record_offset":71928,"ancestor_record_offset":71868,"inlinee":5882,"source_line":1777,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":553,"record_offset":71928,"parent_record_offset":71896,"ancestor_record_offset":71868,"inlinee":27913,"source_line":6,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::child_wall::ChildWall::slice
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":540,"record_offset":111392,"rva":10633216,"va":5379342336}
// 编译实例 RVAs: 0xa24000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32199384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379342336,"raw_hex":"e8d852eb01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379342675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32199248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379342336,"raw_hex":"e85052eb01","size":5,"soft_fail":false,"target_va":5411541904,"va":5379342651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2540680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379342336,"raw_hex":"e888c42600","size":5,"soft_fail":false,"target_va":5381883088,"va":5379342403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":540,"record_offset":111588,"parent_record_offset":111392,"ancestor_record_offset":111392,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":540,"record_offset":111532,"parent_record_offset":111392,"ancestor_record_offset":111392,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":540,"record_offset":111672,"parent_record_offset":111648,"ancestor_record_offset":111392,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":540,"record_offset":111696,"parent_record_offset":111672,"ancestor_record_offset":111392,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":540,"record_offset":111732,"parent_record_offset":111588,"ancestor_record_offset":111392,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":540,"record_offset":111760,"parent_record_offset":111588,"ancestor_record_offset":111392,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":540,"record_offset":111648,"parent_record_offset":111624,"ancestor_record_offset":111392,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":540,"record_offset":111624,"parent_record_offset":111588,"ancestor_record_offset":111392,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00540.jsonl.gz"}
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
// 完整结构及原始类型记录: child_wall.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::child_wall::ChildWall
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["country_core::utils::child_wall::ChildWall", "country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::merge::merge<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0> >", "constructor": "core::slice::sort::stable::merge::merge", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::utils::child_wall::ChildWall,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0>,alloc::vec::Vec<country_core::utils::child_wall::ChildWall,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0>", "alloc::vec::Vec<country_core::utils::child_wall::ChildWall,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort8_stable<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::sort8_stable", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$3::finish_shrink_or_split_event::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: child_wall.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: child_wall.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::utils::child_wall::ChildWall::new
// 信息 ID: 13404cb99fca3847e668e345
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=553, inline_record=71868; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=84984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::child_wall::ChildWall::slice
// 信息 ID: d251cbe99c5ed4b43b22853a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::eraser::archivist::WallEraserArchivist::shrink_wall @ 0x92f1fb -> 0xa24000 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::archivist::WallEraserArchivist::add_split_event @ 0x92f502 -> 0xa24000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
