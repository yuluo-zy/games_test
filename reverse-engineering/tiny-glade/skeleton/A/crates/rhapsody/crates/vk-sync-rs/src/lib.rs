// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/crates/vk-sync-rs/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: acd1081b9e8406a6249a48024b500a05b51bbd56200e1dbd6b43419c5a8b3094
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 195, 467, 558, 560, 583
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use vk_sync_fork; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: vk_sync_fork::impl$1::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":195,"record_offset":17264,"rva":3127184,"va":5371836304}
// 编译实例 RVAs: 0x2fb790, 0x64bbc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t5747577","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5371836304,"raw_hex":"e979b35700","size":5,"soft_fail":false,"target_va":5377583920,"va":5371836338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: vk_sync_fork::impl$5::eq
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3133
// 首条位置: {"module":195,"inlinee":12595}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: vk_sync_fork::get_memory_barrier
// 记录形式: compiled_procedure; instances=1
// 源码行: 317
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":583,"record_offset":252,"rva":7652176,"va":5376361296}
// 编译实例 RVAs: 0x74c350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] vk_sync_fork::get_access_info
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376361296,"raw_hex":"e8a0050000","size":5,"soft_fail":false,"target_va":5376362832,"va":5376361387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$669::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":583,"record_offset":720,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":31673,"source_line":422,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$677::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":583,"record_offset":656,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":30731,"source_line":45,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":583,"record_offset":380,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] vk_sync_fork::AccessType::is_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/vk-sync-rs/src/lib.rs
//     evidence={"module":583,"record_offset":456,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":31671,"source_line":1474,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$676::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":583,"record_offset":684,"parent_record_offset":656,"ancestor_record_offset":252,"inlinee":30733,"source_line":39,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":583,"record_offset":620,"parent_record_offset":596,"ancestor_record_offset":252,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":583,"record_offset":420,"parent_record_offset":380,"ancestor_record_offset":252,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":583,"record_offset":596,"parent_record_offset":484,"ancestor_record_offset":252,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: vk_sync_fork::get_buffer_memory_barrier
// 记录形式: compiled_procedure; instances=1
// 源码行: 416
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":583,"record_offset":752,"rva":7652560,"va":5376361680}
// 编译实例 RVAs: 0x74c4d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] vk_sync_fork::get_access_info
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376361680,"raw_hex":"e800040000","size":5,"soft_fail":false,"target_va":5376362832,"va":5376361803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$669::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":583,"record_offset":1228,"parent_record_offset":752,"ancestor_record_offset":752,"inlinee":31673,"source_line":422,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$677::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":583,"record_offset":1164,"parent_record_offset":752,"ancestor_record_offset":752,"inlinee":30731,"source_line":45,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":583,"record_offset":888,"parent_record_offset":752,"ancestor_record_offset":752,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] vk_sync_fork::AccessType::is_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/vk-sync-rs/src/lib.rs
//     evidence={"module":583,"record_offset":964,"parent_record_offset":752,"ancestor_record_offset":752,"inlinee":31671,"source_line":1474,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$676::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":583,"record_offset":1192,"parent_record_offset":1164,"ancestor_record_offset":752,"inlinee":30733,"source_line":39,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":583,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":752,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":583,"record_offset":928,"parent_record_offset":888,"ancestor_record_offset":752,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":583,"record_offset":1104,"parent_record_offset":992,"ancestor_record_offset":752,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: vk_sync_fork::get_image_memory_barrier
// 记录形式: compiled_procedure; instances=1
// 源码行: 529
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":583,"record_offset":1260,"rva":7652992,"va":5376362112}
// 编译实例 RVAs: 0x74c680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26616345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362112,"raw_hex":"e819229601","size":5,"soft_fail":false,"target_va":5402979152,"va":5376362802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] vk_sync_fork::get_access_info
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362112,"raw_hex":"e84e020000","size":5,"soft_fail":false,"target_va":5376362832,"va":5376362237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$269::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":583,"record_offset":1720,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":31681,"source_line":45,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$669::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":583,"record_offset":1804,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":31673,"source_line":422,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$677::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":583,"record_offset":1664,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":30731,"source_line":45,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":583,"record_offset":1404,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] vk_sync_fork::AccessType::is_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/vk-sync-rs/src/lib.rs
//     evidence={"module":583,"record_offset":1516,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":31671,"source_line":1474,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] vk_sync_fork::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/vk-sync-rs/src/lib.rs
//     evidence={"module":583,"record_offset":1776,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":12595,"source_line":21,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$268::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":583,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":1260,"inlinee":31683,"source_line":39,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$676::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":583,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":1260,"inlinee":30733,"source_line":39,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":583,"record_offset":1472,"parent_record_offset":1404,"ancestor_record_offset":1260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00583.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: vk_sync_fork::get_access_info
// 记录形式: compiled_procedure; instances=1
// 源码行: 713
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":583,"record_offset":1836,"rva":7653712,"va":5376362832}
// 编译实例 RVAs: 0x74c950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: vk_sync_fork::AccessType::is_write
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 1474, 1475
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7BB7
// 首条位置: {"module":583,"inlinee":31671}
// 编译实例 RVAs: 0x74c990
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>

// ------------------------------------------------------------------
// 函数: vk_sync_fork::AccessType::is_write
// 信息 ID: f33eef6739d205255d943def
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 47 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64945 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64a58 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64ce3 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64bf2 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1cc869 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1cc988 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1ccc13 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1ccb22 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1cd1d5 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1cd2e8 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1cd573 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1cd482 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x338a25 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x338b38 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x338dc3 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x338cd2 -> 0x74c990 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] vk_sync_fork::get_memory_barrier; module=583, inline_record=456; 不是独立调用边。
// [PDB 内联宿主] vk_sync_fork::get_buffer_memory_barrier; module=583, inline_record=964; 不是独立调用边。
// [PDB 内联宿主] vk_sync_fork::get_image_memory_barrier; module=583, inline_record=1516; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: vk_sync_fork::get_access_info
// 信息 ID: f43c7a9beedf5bfcfce5c07a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x19824 -> 0x74c950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::extend_usage_flags_for_exports @ 0x72f15a -> 0x74c950 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_memory_barrier @ 0x74c3ab -> 0x74c950 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_memory_barrier @ 0x74c3f8 -> 0x74c950 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_memory_barrier @ 0x74c45b -> 0x74c950 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_buffer_memory_barrier @ 0x74c54b -> 0x74c950 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_buffer_memory_barrier @ 0x74c599 -> 0x74c950 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_buffer_memory_barrier @ 0x74c5fb -> 0x74c950 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_image_memory_barrier @ 0x74c6fd -> 0x74c950 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_image_memory_barrier @ 0x74c86d -> 0x74c950 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_image_memory_barrier @ 0x74c78b -> 0x74c950 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_image_memory_barrier @ 0x74c7d9 -> 0x74c950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::binding::rg_binding_to_descriptor_binding @ 0x18020cc -> 0x74c950 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::DescriptorImageInfo,core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2> > > > @ 0x19153eb -> 0x74c950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: vk_sync_fork::get_buffer_memory_barrier
// 信息 ID: 7e81bf2d39dd4ec58c8afea5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::barrier::impl$5::buffer_barrier @ 0x7336b3 -> 0x74c4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::cmd::pipeline_barrier @ 0x118752d -> 0x74c4d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: vk_sync_fork::get_image_memory_barrier
// 信息 ID: 727c5ec571b25488961b70d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::barrier::impl$5::image_barrier @ 0x733551 -> 0x74c680 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::cmd::pipeline_barrier @ 0x1187606 -> 0x74c680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: vk_sync_fork::get_memory_barrier
// 信息 ID: d3284750be2dd9a019c2c3b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] vk_sync_fork::cmd::pipeline_barrier @ 0x118742b -> 0x74c350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: vk_sync_fork::impl$1::fmt
// 信息 ID: a4878b4ca275f1e4ffdc1f82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: vk_sync_fork::impl$5::eq
// 信息 ID: 31305099d19a4c03db5525ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::pass_builder::PassBuilder::add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>; module=195, inline_record=19104; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::pass_builder::PassBuilder::add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>; module=467, inline_record=10804; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::extend_usage_flags_for_exports; module=558, inline_record=1808; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::recorder::PassResourceAccessType::can_be_concurrent_with; module=560, inline_record=1048; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::recorder::PassResourceAccessType::can_be_concurrent_with; module=560, inline_record=1100; 不是独立调用边。
// [PDB 内联宿主] vk_sync_fork::get_image_memory_barrier; module=583, inline_record=1776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
