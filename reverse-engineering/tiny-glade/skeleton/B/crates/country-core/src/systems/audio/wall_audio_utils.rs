// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/audio/wall_audio_utils.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c48ef5ad5773dd295590cd4d3bb0e65fcfed7350772e0e8999ee8102badceee4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_audio_utils.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 507, 513
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
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":507,"record_offset":33196,"rva":9602464,"va":5378311584}
// 编译实例 RVAs: 0x9285a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a526b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1220569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311584,"raw_hex":"e8d99f1200","size":5,"soft_fail":false,"target_va":5379532464,"va":5378311890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a57670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1241182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311584,"raw_hex":"e85ef01200","size":5,"soft_fail":false,"target_va":5379552880,"va":5378311693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a57da0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1243046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311584,"raw_hex":"e8a6f71200","size":5,"soft_fail":false,"target_va":5379554720,"va":5378311669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33230522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311584,"raw_hex":"e8ba0efb01","size":5,"soft_fail":false,"target_va":5411542296,"va":5378311769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33231034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311584,"raw_hex":"e8ba10fb01","size":5,"soft_fail":false,"target_va":5411542880,"va":5378311841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::set_volume
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11592981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311584,"raw_hex":"e815e5b000","size":5,"soft_fail":false,"target_va":5389904704,"va":5378311718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::audio_resources::impl$10::set_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":507,"record_offset":33504,"parent_record_offset":33196,"ancestor_record_offset":33196,"inlinee":28845,"source_line":615,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::audio_utils::AudioLoop::current
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/audio_utils.rs
//     evidence={"module":507,"record_offset":33364,"parent_record_offset":33196,"ancestor_record_offset":33196,"inlinee":28843,"source_line":95,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":33392,"parent_record_offset":33364,"ancestor_record_offset":33196,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":507,"record_offset":33536,"parent_record_offset":33504,"ancestor_record_offset":33196,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":507,"record_offset":33564,"parent_record_offset":33504,"ancestor_record_offset":33196,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":507,"record_offset":33424,"parent_record_offset":33392,"ancestor_record_offset":33196,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":507,"record_offset":33456,"parent_record_offset":33424,"ancestor_record_offset":33196,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::resolve
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x70D0
// 首条位置: {"module":513,"inlinee":28880}
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
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x70D1
// 首条位置: {"module":513,"inlinee":28881}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::new
// 信息 ID: aada89f984a52eb2a50e8756
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=513, inline_record=9960; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=513, inline_record=10088; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=513, inline_record=10216; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=513, inline_record=10344; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=513, inline_record=10472; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=513, inline_record=10600; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=513, inline_record=10728; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=513, inline_record=10856; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::request_play
// 信息 ID: 8220d95572d047c0b6b7fef3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x950dfd -> 0x9285a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::resolve
// 信息 ID: 2825ac05a72f9a6668bea5ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=513, inline_record=16552; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=513, inline_record=16644; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=513, inline_record=16736; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=513, inline_record=16828; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=513, inline_record=16920; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=513, inline_record=17012; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=513, inline_record=17104; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=513, inline_record=17196; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
