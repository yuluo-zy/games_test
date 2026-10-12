// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/rng.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 4448416a50949d36fa245e29e7e048ea43fdb8428b53e3f2c1f5bd8264aa9c52
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: rng.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 764
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::rng::init_rng
// 记录形式: compiled_procedure; instances=1
// 源码行: 2
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":764,"record_offset":252,"rva":13197888,"va":5381907008}
// 编译实例 RVAs: 0xc96240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::rng::init_rng_u64
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rng.rs
//     evidence={"module":764,"record_offset":356,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":44911,"source_line":5,"archive":"_pdb/B/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rng::init_rng_u64
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF6F
// 首条位置: {"module":764,"inlinee":44911}
// 编译实例 RVAs: 0xc96270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: rng.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: rng.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: rng.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::rng::init_rng
// 信息 ID: af568bcf16c03e8b42bc9d47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 22 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7755cf -> 0xc96240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::lantern::maintain_lantern_lights @ 0x790b33 -> 0xc96240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x7c8f07 -> 0xc96240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals @ 0x7e6768 -> 0xc96240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7ed663 -> 0xc96240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7ee2d5 -> 0xc96240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors @ 0x7f4e88 -> 0xc96240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::add_door_autoclutter @ 0x7f5eab -> 0xc96240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x7f7f88 -> 0xc96240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::chimney_smoke_anim_thickness::chimney_smoke_anim_thickness @ 0x8196b0 -> 0xc96240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121ae49 -> 0xc96240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121bd89 -> 0xc96240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x124c24c -> 0xc96240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x1250352 -> 0xc96240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker::with_diffs<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams,system_decorator::decorator_visual::chimney_assemble_bricks::chimney_assemble_bricks::closure_env$0> @ 0x12aebfc -> 0xc96240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker::with_diffs<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams,system_decorator::decorator_visual::door_stairs_assemble_bricks::door_stairs_assemble_bricks::closure_env$0> @ 0x12af856 -> 0xc96240 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rng::init_rng_u64
// 信息 ID: 7c0ff8d001b485628c7e39cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0x93df -> 0xc96270 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x186e1 -> 0xc96270 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22d7eb1 -> 0xc96270 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22d846f -> 0xc96270 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::rng::init_rng; module=764, inline_record=356; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
