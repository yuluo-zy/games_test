// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/cottage_dormers/mantain_dormer_tiles_draw.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 500b2b4707b2722b078c21cc165307db550468f51c25c93fc16d88dc81f28b8c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mantain_dormer_tiles_draw.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3848
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_dormers::mantain_dormer_tiles_draw::mantain_dormer_tiles
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3848,"record_offset":256,"rva":29636624,"va":5398345744}
// 编译实例 RVAs: 0x1c43810
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: mantain_dormer_tiles_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28732481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398345744,"raw_hex":"e8bf9349fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5398345996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14074c200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mantain_dormer_tiles_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21985063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398345744,"raw_hex":"e8d988b0fe","size":5,"soft_fail":false,"target_va":5376360960,"va":5398346018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d01bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mantain_dormer_tiles_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15998344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398345744,"raw_hex":"e878e20bff","size":5,"soft_fail":false,"target_va":5382347712,"va":5398346051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3848,"record_offset":1368,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3848,"record_offset":456,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3848,"record_offset":996,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3848,"record_offset":1548,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3848,"record_offset":1424,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":7345,"source_line":1160,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3848,"record_offset":1392,"parent_record_offset":1368,"ancestor_record_offset":256,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3848,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":256,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3848,"record_offset":520,"parent_record_offset":480,"ancestor_record_offset":256,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3848,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":256,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3848,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":256,"inlinee":10427,"source_line":846,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3848,"record_offset":480,"parent_record_offset":456,"ancestor_record_offset":256,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3848,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":256,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3848,"record_offset":1052,"parent_record_offset":1020,"ancestor_record_offset":256,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3848,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":256,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3848,"record_offset":588,"parent_record_offset":544,"ancestor_record_offset":256,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3848,"record_offset":1100,"parent_record_offset":1076,"ancestor_record_offset":256,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3848,"record_offset":1304,"parent_record_offset":1020,"ancestor_record_offset":256,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3848,"record_offset":1132,"parent_record_offset":1052,"ancestor_record_offset":256,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3848,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":256,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3848,"record_offset":1256,"parent_record_offset":1232,"ancestor_record_offset":256,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3848,"record_offset":652,"parent_record_offset":544,"ancestor_record_offset":256,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3848,"record_offset":840,"parent_record_offset":544,"ancestor_record_offset":256,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3848,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":256,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3848,"record_offset":688,"parent_record_offset":652,"ancestor_record_offset":256,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3848,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":256,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3848,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":256,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3848,"record_offset":1208,"parent_record_offset":1156,"ancestor_record_offset":256,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3848,"record_offset":1452,"parent_record_offset":1424,"ancestor_record_offset":256,"inlinee":7346,"source_line":1895,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3848,"record_offset":1480,"parent_record_offset":1452,"ancestor_record_offset":256,"inlinee":7347,"source_line":1934,"archive":"_pdb/A/modules/03848.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3848,"record_offset":1508,"parent_record_offset":1480,"ancestor_record_offset":256,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03848.jsonl.gz"}
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
// 完整结构及原始类型记录: mantain_dormer_tiles_draw.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mantain_dormer_tiles_draw.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mantain_dormer_tiles_draw.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>", "bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_dormers::mantain_dormer_tiles_draw::mantain_dormer_tiles
// 信息 ID: 40c80b4f45025814aabb836d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] df07f0cd54a8886156f4323d: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>)> > @ 0x17d01a0 -> 0x1c43810 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>)> @ 0x17e2f69 -> 0x1c43810 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
