// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/terrain_rock_mask.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: db76912950f42d3b84c56e12b6e3c44cfcf6c916634f31abfaf3dbc43d134b22
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: terrain_rock_mask.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 605, 1686, 3273
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_rock_mask::impl$1::sample_at_world_xz
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x96EC
// 首条位置: {"module":605,"record_offset":58192,"rva":13046976,"va":5381756096}
// 编译实例 RVAs: 0xc714c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34865450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381756096,"raw_hex":"e82a011402","size":5,"soft_fail":false,"target_va":5416621762,"va":5381756307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":605,"record_offset":58640,"parent_record_offset":58192,"ancestor_record_offset":58192,"inlinee":8904,"source_line":1088,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":605,"record_offset":58464,"parent_record_offset":58192,"ancestor_record_offset":58192,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":605,"record_offset":58560,"parent_record_offset":58192,"ancestor_record_offset":58192,"inlinee":15311,"source_line":1259,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":605,"record_offset":58368,"parent_record_offset":58192,"ancestor_record_offset":58192,"inlinee":8895,"source_line":1124,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$40::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":605,"record_offset":58416,"parent_record_offset":58192,"ancestor_record_offset":58192,"inlinee":21956,"source_line":1444,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::min
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec2.rs
//     evidence={"module":605,"record_offset":58680,"parent_record_offset":58192,"ancestor_record_offset":58192,"inlinee":8909,"source_line":172,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":605,"record_offset":58436,"parent_record_offset":58416,"ancestor_record_offset":58192,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":605,"record_offset":58588,"parent_record_offset":58560,"ancestor_record_offset":58192,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":605,"record_offset":58388,"parent_record_offset":58368,"ancestor_record_offset":58192,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":605,"record_offset":58504,"parent_record_offset":58464,"ancestor_record_offset":58192,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00605.jsonl.gz"}
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
// 完整结构及原始类型记录: terrain_rock_mask.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo>", "constructor": "country_core::render::ssbo::impl$4::new_storage_buffer", "arguments": ["country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$1::as_any_mut<country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$1::as_any_mut", "arguments": ["country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$1::as_any<country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$1::as_any", "arguments": ["country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::MappedStorageBuffer<country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo>", "constructor": "country_core::render::ssbo::MappedStorageBuffer", "arguments": ["country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::MappedStorageBuffer<country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::MappedStorageBuffer<country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: terrain_rock_mask.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/terrain-editing/source-ownership.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/terrain_rock_mask.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: terrain_rock_mask.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_rock_mask::impl$1::sample_at_world_xz
// 信息 ID: 1c4a3bb91b49906476a56f88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] forest::manage_tree_prefabs::manage_tree_prefabs @ 0x20b434e -> 0xc714c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::butterfly::update_butterfly_states @ 0x2292fe6 -> 0xc714c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::convert_preplaced_trees::convert_preplaced_trees @ 0x2863d6d -> 0xc714c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bf126 -> 0xc714c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::move_critters @ 0x2935f5d -> 0xc714c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::critters_avoid_collisions @ 0x293736e -> 0xc714c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::sheep::sheep_unstuck::sheep_unstuck @ 0x294396e -> 0xc714c0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::terrain::ui_draw_terrain::ui_draw_terrain; module=1686, inline_record=5584; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::systems::nani_update_preplaced_flower; module=3273, inline_record=172672; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
