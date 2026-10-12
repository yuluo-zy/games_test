// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/usdl-sys/src/keyboard.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: f02172b51a3d0b2d0b9d6dcd2498b8aca8fab8119f97564521dccff8e69634ae
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: keyboard.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1012
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use usdl_sys; // namespace candidate
//   package identities: [{"name":"usdl-sys","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: usdl_sys::keyboard::uSDL_GetModState
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1012,"record_offset":724,"rva":16785184,"va":5385494304}
// 编译实例 RVAs: 0x1001f20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31127850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385494304,"raw_hex":"e82af9da01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385494385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*304(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5385494304,"raw_hex":"ff9030010000","size":6,"soft_fail":false,"va":5385494335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1012,"record_offset":844,"parent_record_offset":724,"ancestor_record_offset":724,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1012,"record_offset":960,"parent_record_offset":940,"ancestor_record_offset":724,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1012,"record_offset":980,"parent_record_offset":960,"ancestor_record_offset":724,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1012,"record_offset":920,"parent_record_offset":900,"ancestor_record_offset":724,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1012,"record_offset":880,"parent_record_offset":844,"ancestor_record_offset":724,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1012,"record_offset":900,"parent_record_offset":880,"ancestor_record_offset":724,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1012,"record_offset":940,"parent_record_offset":920,"ancestor_record_offset":724,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01012.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::keyboard::uSDL_GetKeyName
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1012,"record_offset":264,"rva":16785088,"va":5385494208}
// 编译实例 RVAs: 0x1001ec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31127948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385494208,"raw_hex":"e88cf9da01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385494287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1012,"record_offset":384,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1012,"record_offset":500,"parent_record_offset":480,"ancestor_record_offset":264,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1012,"record_offset":520,"parent_record_offset":500,"ancestor_record_offset":264,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1012,"record_offset":460,"parent_record_offset":440,"ancestor_record_offset":264,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1012,"record_offset":420,"parent_record_offset":384,"ancestor_record_offset":264,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1012,"record_offset":440,"parent_record_offset":420,"ancestor_record_offset":264,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1012,"record_offset":480,"parent_record_offset":460,"ancestor_record_offset":264,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01012.jsonl.gz"}
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
// 完整结构及原始类型记录: keyboard.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl_sys::keyboard::_::InternalBitFlags
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: keyboard.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: keyboard.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: usdl_sys::keyboard::uSDL_GetKeyName
// 信息 ID: 7df568090005162299b51776
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::keyboard::Keycode::name @ 0x6cb610 -> 0x1001ec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::keyboard::uSDL_GetModState
// 信息 ID: 9933c9d7f062a643f7b093a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::keyboard::KeyboardUtil::mod_state @ 0x6cb5e4 -> 0x1001f20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
