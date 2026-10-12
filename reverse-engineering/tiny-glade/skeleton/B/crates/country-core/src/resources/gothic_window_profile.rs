// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/gothic_window_profile.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: f52c81a50e6ee168e406b0596cb95c88d72ff4379f9d7bc566254f62d6684faa
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gothic_window_profile.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 527
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::gothic_window_profile::gothic_door_profile_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":527,"record_offset":19080,"rva":10112624,"va":5378821744}
// 编译实例 RVAs: 0x9a4e70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: gothic_window_profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3740312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378821744,"raw_hex":"e868edc6ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378821795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32718982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378821744,"raw_hex":"e88640f301","size":5,"soft_fail":false,"target_va":5411540992,"va":5378822005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32720030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378821744,"raw_hex":"e89e44f301","size":5,"soft_fail":false,"target_va":5411542064,"va":5378822029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":527,"record_offset":19244,"parent_record_offset":19080,"ancestor_record_offset":19080,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00527.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":527,"record_offset":19676,"parent_record_offset":19080,"ancestor_record_offset":19080,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":527,"record_offset":19292,"parent_record_offset":19268,"ancestor_record_offset":19080,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":527,"record_offset":19316,"parent_record_offset":19292,"ancestor_record_offset":19080,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":527,"record_offset":19268,"parent_record_offset":19244,"ancestor_record_offset":19080,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":527,"record_offset":19712,"parent_record_offset":19676,"ancestor_record_offset":19080,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":527,"record_offset":19736,"parent_record_offset":19712,"ancestor_record_offset":19080,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00527.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":527,"record_offset":19340,"parent_record_offset":19316,"ancestor_record_offset":19080,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00527.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":527,"record_offset":18372,"rva":10112192,"va":5378821312}
// 编译实例 RVAs: 0x9a4cc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: gothic_window_profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3739880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378821312,"raw_hex":"e818efc6ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378821363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32719293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378821312,"raw_hex":"e8bd41f301","size":5,"soft_fail":false,"target_va":5411540992,"va":5378821694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32720380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378821312,"raw_hex":"e8fc45f301","size":5,"soft_fail":false,"target_va":5411542064,"va":5378821679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":527,"record_offset":18540,"parent_record_offset":18372,"ancestor_record_offset":18372,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00527.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":527,"record_offset":18992,"parent_record_offset":18372,"ancestor_record_offset":18372,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":527,"record_offset":18596,"parent_record_offset":18572,"ancestor_record_offset":18372,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":527,"record_offset":18620,"parent_record_offset":18596,"ancestor_record_offset":18372,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":527,"record_offset":18572,"parent_record_offset":18540,"ancestor_record_offset":18372,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":527,"record_offset":19016,"parent_record_offset":18992,"ancestor_record_offset":18372,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":527,"record_offset":19040,"parent_record_offset":19016,"ancestor_record_offset":18372,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00527.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":527,"record_offset":18644,"parent_record_offset":18620,"ancestor_record_offset":18372,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00527.jsonl.gz"}
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
// 完整结构及原始类型记录: gothic_window_profile.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gothic_window_profile.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gothic_window_profile.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::resources::gothic_window_profile::gothic_door_profile_curve
// 信息 ID: f5e33e3f9dc1c310a0dd3e7f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121c6d7 -> 0x9a4e70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::get_window_aabb @ 0x1221df5 -> 0x9a4e70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve
// 信息 ID: da4dc76c5e8d0aa9e739f8e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs @ 0x7cc8cd -> 0x9a4cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds @ 0x811593 -> 0x9a4cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121c6cd -> 0x9a4cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::get_window_aabb @ 0x1221e71 -> 0x9a4cc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
