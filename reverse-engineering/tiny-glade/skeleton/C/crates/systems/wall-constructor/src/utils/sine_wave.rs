// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/utils/sine_wave.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 31fc9ccda15a88bf09e6eec877cbf974643ef8e26dbe0f36095ea806dc923e4d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sine_wave.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 427, 1493
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::sine_wave::BrickRowSine::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6813
// 首条位置: {"module":427,"record_offset":280,"rva":8660000,"va":5377369120}
// 编译实例 RVAs: 0x842420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: sine_wave.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39688773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377369120,"raw_hex":"e8459a5d02","size":5,"soft_fail":false,"target_va":5417058032,"va":5377369254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bf80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: sine_wave.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39688862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377369120,"raw_hex":"e89e9a5d02","size":5,"soft_fail":false,"target_va":5417058176,"va":5377369309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::random_f
//     owner=private_source_ownership_candidate; count=20
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":427,"record_offset":424,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26639,"source_line":82,"archive":"_pdb/C/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::fract
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":427,"record_offset":512,"parent_record_offset":484,"ancestor_record_offset":280,"inlinee":26641,"source_line":1873,"archive":"_pdb/C/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::trunc
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":427,"record_offset":540,"parent_record_offset":512,"ancestor_record_offset":280,"inlinee":21635,"source_line":1844,"archive":"_pdb/C/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::fract
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":427,"record_offset":484,"parent_record_offset":424,"ancestor_record_offset":280,"inlinee":26640,"source_line":178,"archive":"_pdb/C/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":427,"record_offset":456,"parent_record_offset":424,"ancestor_record_offset":280,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00427.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::sine_wave::sine_wave_deform
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=10
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6815
// 首条位置: {"module":427,"record_offset":1164,"rva":8660432,"va":5377369552}
// 编译实例 RVAs: 0x8425d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: sine_wave.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39688427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377369552,"raw_hex":"e8eb985d02","size":5,"soft_fail":false,"target_va":5417058032,"va":5377369600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":427,"record_offset":1308,"parent_record_offset":1164,"ancestor_record_offset":1164,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00427.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::sine_wave::sine_wave_pack
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6814
// 首条位置: {"module":427,"record_offset":1028,"rva":8660400,"va":5377369520}
// 编译实例 RVAs: 0x8425b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::sine_wave::random_f
// 记录形式: inline_instance, inlinee_declaration; instances=22
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x680F
// 首条位置: {"module":427,"inlinee":26639}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::fract
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":427,"record_offset":484,"parent_record_offset":424,"ancestor_record_offset":424,"inlinee":26640,"source_line":178,"archive":"_pdb/C/modules/00427.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":427,"record_offset":456,"parent_record_offset":424,"ancestor_record_offset":424,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::fract
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":427,"record_offset":512,"parent_record_offset":484,"ancestor_record_offset":424,"inlinee":26641,"source_line":1873,"archive":"_pdb/C/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::trunc
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":427,"record_offset":540,"parent_record_offset":512,"ancestor_record_offset":424,"inlinee":21635,"source_line":1844,"archive":"_pdb/C/modules/00427.jsonl.gz"}
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
// 完整结构及原始类型记录: sine_wave.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::utils::sine_wave::BrickRowSine
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sine_wave.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sine_wave.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::sine_wave::BrickRowSine::new
// 信息 ID: 783147eb3c2cd1625c8e37c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x679e59 -> 0x842420 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>; module=1493, inline_record=15884; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1493, inline_record=37680; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1493, inline_record=66308; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>; module=1493, inline_record=88356; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::sine_wave::random_f
// 信息 ID: d2275a255a9911bfa8baf394
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::sine_wave::BrickRowSine::new; module=427, inline_record=424; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::sine_wave::BrickRowSine::new; module=427, inline_record=580; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::sine_wave::BrickRowSine::new; module=427, inline_record=724; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::sine_wave::BrickRowSine::new; module=427, inline_record=880; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>; module=1493, inline_record=15932; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>; module=1493, inline_record=16072; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>; module=1493, inline_record=16212; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>; module=1493, inline_record=16352; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::sine_wave::sine_wave_deform
// 信息 ID: fa17deec634edff75e49f131
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x67a594 -> 0x8425d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x67a578 -> 0x8425d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>; module=1493, inline_record=19076; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>; module=1493, inline_record=19224; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1493, inline_record=40300; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1493, inline_record=40448; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1493, inline_record=69500; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1493, inline_record=69648; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>; module=1493, inline_record=90976; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>; module=1493, inline_record=91124; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::sine_wave::sine_wave_pack
// 信息 ID: 4af4718bf06740b3b2729b74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x67a5cd -> 0x8425b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>; module=1493, inline_record=19360; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1493, inline_record=40588; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1493, inline_record=69788; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>; module=1493, inline_record=91264; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
