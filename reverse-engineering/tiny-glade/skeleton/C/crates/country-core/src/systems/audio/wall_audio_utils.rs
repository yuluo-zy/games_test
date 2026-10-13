// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/audio/wall_audio_utils.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: c48ef5ad5773dd295590cd4d3bb0e65fcfed7350772e0e8999ee8102badceee4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_audio_utils.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 538, 1774
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use libfmod; // namespace candidate
//   package identities: [{"name":"libfmod","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.222.6"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::request_play
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8147
// 首条位置: {"module":538,"record_offset":104716,"rva":10889616,"va":5379598736}
// 编译实例 RVAs: 0xa62990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7b7d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t101858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379598736,"raw_hex":"e8e28d0100","size":5,"soft_fail":false,"target_va":5379700688,"va":5379598825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7bd70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t103274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379598736,"raw_hex":"e86a930100","size":5,"soft_fail":false,"target_va":5379702128,"va":5379598849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7cbd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t106761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379598736,"raw_hex":"e809a10100","size":5,"soft_fail":false,"target_va":5379705808,"va":5379599042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37022757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379598736,"raw_hex":"e825ec3402","size":5,"soft_fail":false,"target_va":5416621762,"va":5379599000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37022645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379598736,"raw_hex":"e8b5eb3402","size":5,"soft_fail":false,"target_va":5416621632,"va":5379598982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::set_volume
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17051553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379598736,"raw_hex":"e8a12f0401","size":5,"soft_fail":false,"target_va":5396650432,"va":5379598874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::audio_resources::impl$10::set_volume
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":538,"record_offset":105024,"parent_record_offset":104716,"ancestor_record_offset":104716,"inlinee":33034,"source_line":615,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::audio_utils::AudioLoop::current
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/audio_utils.rs
//     evidence={"module":538,"record_offset":104884,"parent_record_offset":104716,"ancestor_record_offset":104716,"inlinee":29641,"source_line":95,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":104912,"parent_record_offset":104884,"ancestor_record_offset":104716,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":538,"record_offset":105056,"parent_record_offset":105024,"ancestor_record_offset":104716,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":538,"record_offset":105084,"parent_record_offset":105024,"ancestor_record_offset":104716,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":538,"record_offset":104944,"parent_record_offset":104912,"ancestor_record_offset":104716,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":538,"record_offset":104976,"parent_record_offset":104944,"ancestor_record_offset":104716,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::resolve
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8149
// 首条位置: {"module":1774,"inlinee":33097}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::new
// 记录形式: inline_instance, inlinee_declaration; instances=15
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8148
// 首条位置: {"module":1774,"inlinee":33096}
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
// 完整结构及原始类型记录: wall_audio_utils.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::audio::wall_audio_utils::AudioLoopPlayer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_audio_utils.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_audio_utils.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::new
// 信息 ID: 7b4f38817695369a4f7ff5b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 14 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=1774, inline_record=52912; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=1774, inline_record=53040; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=1774, inline_record=53168; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=1774, inline_record=53296; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=1774, inline_record=53424; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=1774, inline_record=53552; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=1774, inline_record=53680; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=1774, inline_record=53808; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::request_play
// 信息 ID: 00c8fbb26160368f02e8a64d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x1aefe1f -> 0xa62990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x1af0c0b -> 0xa62990 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=1774, inline_record=46608; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::resolve
// 信息 ID: 276c4d95a09d79db41cb8d1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=1774, inline_record=47332; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=1774, inline_record=47424; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=1774, inline_record=47516; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=1774, inline_record=47608; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=1774, inline_record=47700; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=1774, inline_record=47792; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=1774, inline_record=47884; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=1774, inline_record=47976; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
