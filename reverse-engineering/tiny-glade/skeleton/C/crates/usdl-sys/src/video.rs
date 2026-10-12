// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/usdl-sys/src/video.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 6296a432971fdb89c39c954b51288a44c55e1eeefc1d699ac4d3490dea52e825
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: video.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 177, 285, 1008
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
// 函数符号: usdl_sys::video::impl$4::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A8E
// 首条位置: {"module":285,"inlinee":19086}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::impl$11::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3BC5
// 首条位置: {"module":177,"inlinee":15301}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_GetCurrentVideoDriver
// 记录形式: compiled_procedure; instances=1
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":8120,"rva":16784496,"va":5385493616}
// 编译实例 RVAs: 0x1001c70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31128546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385493616,"raw_hex":"e8e2fbda01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385493689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":8248,"parent_record_offset":8120,"ancestor_record_offset":8120,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":8364,"parent_record_offset":8344,"ancestor_record_offset":8120,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":8384,"parent_record_offset":8364,"ancestor_record_offset":8120,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":8324,"parent_record_offset":8304,"ancestor_record_offset":8120,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":8284,"parent_record_offset":8248,"ancestor_record_offset":8120,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":8304,"parent_record_offset":8284,"ancestor_record_offset":8120,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":8344,"parent_record_offset":8324,"ancestor_record_offset":8120,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_GetNumVideoDisplays
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":6248,"rva":16784128,"va":5385493248}
// 编译实例 RVAs: 0x1001b00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31128914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385493248,"raw_hex":"e852fdda01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385493321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":6376,"parent_record_offset":6248,"ancestor_record_offset":6248,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":6492,"parent_record_offset":6472,"ancestor_record_offset":6248,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":6512,"parent_record_offset":6492,"ancestor_record_offset":6248,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":6452,"parent_record_offset":6432,"ancestor_record_offset":6248,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":6412,"parent_record_offset":6376,"ancestor_record_offset":6248,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":6432,"parent_record_offset":6412,"ancestor_record_offset":6248,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":6472,"parent_record_offset":6452,"ancestor_record_offset":6248,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_GetDisplayName
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":3480,"rva":16783584,"va":5385492704}
// 编译实例 RVAs: 0x10018e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31129458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385492704,"raw_hex":"e872ffda01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385492777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":3600,"parent_record_offset":3480,"ancestor_record_offset":3480,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":3716,"parent_record_offset":3696,"ancestor_record_offset":3480,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":3736,"parent_record_offset":3716,"ancestor_record_offset":3480,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":3676,"parent_record_offset":3656,"ancestor_record_offset":3480,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":3636,"parent_record_offset":3600,"ancestor_record_offset":3480,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":3656,"parent_record_offset":3636,"ancestor_record_offset":3480,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":3696,"parent_record_offset":3676,"ancestor_record_offset":3480,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_GetDisplayBounds
// 记录形式: compiled_procedure; instances=1
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":5320,"rva":16783952,"va":5385493072}
// 编译实例 RVAs: 0x1001a50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31129084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385493072,"raw_hex":"e8fcfdda01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385493151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":5444,"parent_record_offset":5320,"ancestor_record_offset":5320,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":5560,"parent_record_offset":5540,"ancestor_record_offset":5320,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":5580,"parent_record_offset":5560,"ancestor_record_offset":5320,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":5520,"parent_record_offset":5500,"ancestor_record_offset":5320,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":5480,"parent_record_offset":5444,"ancestor_record_offset":5320,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":5500,"parent_record_offset":5480,"ancestor_record_offset":5320,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":5540,"parent_record_offset":5520,"ancestor_record_offset":5320,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_GetNumDisplayModes
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":5784,"rva":16784048,"va":5385493168}
// 编译实例 RVAs: 0x1001ab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31128994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385493168,"raw_hex":"e8a2fdda01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385493241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":5908,"parent_record_offset":5784,"ancestor_record_offset":5784,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":6024,"parent_record_offset":6004,"ancestor_record_offset":5784,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":6044,"parent_record_offset":6024,"ancestor_record_offset":5784,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":5984,"parent_record_offset":5964,"ancestor_record_offset":5784,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":5944,"parent_record_offset":5908,"ancestor_record_offset":5784,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":5964,"parent_record_offset":5944,"ancestor_record_offset":5784,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":6004,"parent_record_offset":5984,"ancestor_record_offset":5784,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_GetDisplayMode
// 记录形式: compiled_procedure; instances=1
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":3020,"rva":16783488,"va":5385492608}
// 编译实例 RVAs: 0x1001880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31129548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385492608,"raw_hex":"e8ccffda01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385492687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":3140,"parent_record_offset":3020,"ancestor_record_offset":3020,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":3256,"parent_record_offset":3236,"ancestor_record_offset":3020,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":3276,"parent_record_offset":3256,"ancestor_record_offset":3020,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":3216,"parent_record_offset":3196,"ancestor_record_offset":3020,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":3176,"parent_record_offset":3140,"ancestor_record_offset":3020,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":3196,"parent_record_offset":3176,"ancestor_record_offset":3020,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":3236,"parent_record_offset":3216,"ancestor_record_offset":3020,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_GetDesktopDisplayMode
// 记录形式: compiled_procedure; instances=1
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":8588,"rva":16784576,"va":5385493696}
// 编译实例 RVAs: 0x1001cc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31128460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385493696,"raw_hex":"e88cfbda01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385493775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":8716,"parent_record_offset":8588,"ancestor_record_offset":8588,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":8832,"parent_record_offset":8812,"ancestor_record_offset":8588,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":8852,"parent_record_offset":8832,"ancestor_record_offset":8588,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":8792,"parent_record_offset":8772,"ancestor_record_offset":8588,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":8752,"parent_record_offset":8716,"ancestor_record_offset":8588,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":8772,"parent_record_offset":8752,"ancestor_record_offset":8588,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":8812,"parent_record_offset":8792,"ancestor_record_offset":8588,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_GetCurrentDisplayMode
// 记录形式: compiled_procedure; instances=1
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":7652,"rva":16784400,"va":5385493520}
// 编译实例 RVAs: 0x1001c10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31128636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385493520,"raw_hex":"e83cfcda01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385493599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":7780,"parent_record_offset":7652,"ancestor_record_offset":7652,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":7896,"parent_record_offset":7876,"ancestor_record_offset":7652,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":7916,"parent_record_offset":7896,"ancestor_record_offset":7652,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":7856,"parent_record_offset":7836,"ancestor_record_offset":7652,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":7816,"parent_record_offset":7780,"ancestor_record_offset":7652,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":7836,"parent_record_offset":7816,"ancestor_record_offset":7652,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":7876,"parent_record_offset":7856,"ancestor_record_offset":7652,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_CreateWindow
// 记录形式: compiled_procedure; instances=1
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":1180,"rva":16783120,"va":5385492240}
// 编译实例 RVAs: 0x1001710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31129916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385492240,"raw_hex":"e83c01db01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385492319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":1300,"parent_record_offset":1180,"ancestor_record_offset":1180,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":1416,"parent_record_offset":1396,"ancestor_record_offset":1180,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":1436,"parent_record_offset":1416,"ancestor_record_offset":1180,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":1376,"parent_record_offset":1356,"ancestor_record_offset":1180,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":1336,"parent_record_offset":1300,"ancestor_record_offset":1180,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":1356,"parent_record_offset":1336,"ancestor_record_offset":1180,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":1396,"parent_record_offset":1376,"ancestor_record_offset":1180,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_DestroyWindow
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":1640,"rva":16783216,"va":5385492336}
// 编译实例 RVAs: 0x1001770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31129826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385492336,"raw_hex":"e8e200db01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385492409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":1760,"parent_record_offset":1640,"ancestor_record_offset":1640,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":1876,"parent_record_offset":1856,"ancestor_record_offset":1640,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":1896,"parent_record_offset":1876,"ancestor_record_offset":1640,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":1836,"parent_record_offset":1816,"ancestor_record_offset":1640,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":1796,"parent_record_offset":1760,"ancestor_record_offset":1640,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":1816,"parent_record_offset":1796,"ancestor_record_offset":1640,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":1856,"parent_record_offset":1836,"ancestor_record_offset":1640,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_ShowWindow
// 记录形式: compiled_procedure; instances=1
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":264,"rva":16782928,"va":5385492048}
// 编译实例 RVAs: 0x1001650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31130108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385492048,"raw_hex":"e8fc01db01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385492127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":380,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":496,"parent_record_offset":476,"ancestor_record_offset":264,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":516,"parent_record_offset":496,"ancestor_record_offset":264,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":456,"parent_record_offset":436,"ancestor_record_offset":264,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":416,"parent_record_offset":380,"ancestor_record_offset":264,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":436,"parent_record_offset":416,"ancestor_record_offset":264,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":476,"parent_record_offset":456,"ancestor_record_offset":264,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_MaximizeWindow
// 记录形式: compiled_procedure; instances=1
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":4400,"rva":16783760,"va":5385492880}
// 编译实例 RVAs: 0x1001990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31129276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385492880,"raw_hex":"e8bcfeda01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385492959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":4520,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":4636,"parent_record_offset":4616,"ancestor_record_offset":4400,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":4656,"parent_record_offset":4636,"ancestor_record_offset":4400,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":4596,"parent_record_offset":4576,"ancestor_record_offset":4400,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":4556,"parent_record_offset":4520,"ancestor_record_offset":4400,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":4576,"parent_record_offset":4556,"ancestor_record_offset":4400,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":4616,"parent_record_offset":4596,"ancestor_record_offset":4400,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_MinimizeWindow
// 记录形式: compiled_procedure; instances=1
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":4860,"rva":16783856,"va":5385492976}
// 编译实例 RVAs: 0x10019f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31129180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385492976,"raw_hex":"e85cfeda01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385493055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":4980,"parent_record_offset":4860,"ancestor_record_offset":4860,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":5096,"parent_record_offset":5076,"ancestor_record_offset":4860,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":5116,"parent_record_offset":5096,"ancestor_record_offset":4860,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":5056,"parent_record_offset":5036,"ancestor_record_offset":4860,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":5016,"parent_record_offset":4980,"ancestor_record_offset":4860,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":5036,"parent_record_offset":5016,"ancestor_record_offset":4860,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":5076,"parent_record_offset":5056,"ancestor_record_offset":4860,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_SetWindowGrab
// 记录形式: compiled_procedure; instances=1
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":2560,"rva":16783392,"va":5385492512}
// 编译实例 RVAs: 0x1001820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31129638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385492512,"raw_hex":"e82600db01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385492597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":2680,"parent_record_offset":2560,"ancestor_record_offset":2560,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":2796,"parent_record_offset":2776,"ancestor_record_offset":2560,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":2816,"parent_record_offset":2796,"ancestor_record_offset":2560,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":2756,"parent_record_offset":2736,"ancestor_record_offset":2560,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":2716,"parent_record_offset":2680,"ancestor_record_offset":2560,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":2736,"parent_record_offset":2716,"ancestor_record_offset":2560,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":2776,"parent_record_offset":2756,"ancestor_record_offset":2560,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_GetWindowDisplayIndex
// 记录形式: compiled_procedure; instances=1
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":9056,"rva":16784672,"va":5385493792}
// 编译实例 RVAs: 0x1001d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31128364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385493792,"raw_hex":"e82cfbda01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385493871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":9184,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":9300,"parent_record_offset":9280,"ancestor_record_offset":9056,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":9320,"parent_record_offset":9300,"ancestor_record_offset":9056,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":9260,"parent_record_offset":9240,"ancestor_record_offset":9056,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":9220,"parent_record_offset":9184,"ancestor_record_offset":9056,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":9240,"parent_record_offset":9220,"ancestor_record_offset":9056,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":9280,"parent_record_offset":9260,"ancestor_record_offset":9056,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_GetWindowID
// 记录形式: compiled_procedure; instances=1
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":720,"rva":16783024,"va":5385492144}
// 编译实例 RVAs: 0x10016b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31130012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385492144,"raw_hex":"e89c01db01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385492223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":840,"parent_record_offset":720,"ancestor_record_offset":720,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":956,"parent_record_offset":936,"ancestor_record_offset":720,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":976,"parent_record_offset":956,"ancestor_record_offset":720,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":916,"parent_record_offset":896,"ancestor_record_offset":720,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":876,"parent_record_offset":840,"ancestor_record_offset":720,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":896,"parent_record_offset":876,"ancestor_record_offset":720,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":936,"parent_record_offset":916,"ancestor_record_offset":720,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_GetWindowSize
// 记录形式: compiled_procedure; instances=1
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":2100,"rva":16783296,"va":5385492416}
// 编译实例 RVAs: 0x10017c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31129734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385492416,"raw_hex":"e88600db01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385492501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":2220,"parent_record_offset":2100,"ancestor_record_offset":2100,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":2336,"parent_record_offset":2316,"ancestor_record_offset":2100,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":2356,"parent_record_offset":2336,"ancestor_record_offset":2100,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":2296,"parent_record_offset":2276,"ancestor_record_offset":2100,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":2256,"parent_record_offset":2220,"ancestor_record_offset":2100,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":2276,"parent_record_offset":2256,"ancestor_record_offset":2100,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":2316,"parent_record_offset":2296,"ancestor_record_offset":2100,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_Vulkan_GetDrawableSize
// 记录形式: compiled_procedure; instances=1
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":9524,"rva":16784768,"va":5385493888}
// 编译实例 RVAs: 0x1001d80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31128262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385493888,"raw_hex":"e8c6fada01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385493973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":9652,"parent_record_offset":9524,"ancestor_record_offset":9524,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":9768,"parent_record_offset":9748,"ancestor_record_offset":9524,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":9788,"parent_record_offset":9768,"ancestor_record_offset":9524,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":9728,"parent_record_offset":9708,"ancestor_record_offset":9524,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":9688,"parent_record_offset":9652,"ancestor_record_offset":9524,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":9708,"parent_record_offset":9688,"ancestor_record_offset":9524,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":9748,"parent_record_offset":9728,"ancestor_record_offset":9524,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_SetWindowDisplayMode
// 记录形式: compiled_procedure; instances=1
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":7184,"rva":16784304,"va":5385493424}
// 编译实例 RVAs: 0x1001bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31128726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385493424,"raw_hex":"e896fcda01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385493509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":7312,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":7428,"parent_record_offset":7408,"ancestor_record_offset":7184,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":7448,"parent_record_offset":7428,"ancestor_record_offset":7184,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":7388,"parent_record_offset":7368,"ancestor_record_offset":7184,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":7348,"parent_record_offset":7312,"ancestor_record_offset":7184,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":7368,"parent_record_offset":7348,"ancestor_record_offset":7184,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":7408,"parent_record_offset":7388,"ancestor_record_offset":7184,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_SetWindowFullscreen
// 记录形式: compiled_procedure; instances=1
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":6716,"rva":16784208,"va":5385493328}
// 编译实例 RVAs: 0x1001b50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31128822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385493328,"raw_hex":"e8f6fcda01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385493413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":6844,"parent_record_offset":6716,"ancestor_record_offset":6716,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":6960,"parent_record_offset":6940,"ancestor_record_offset":6716,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":6980,"parent_record_offset":6960,"ancestor_record_offset":6716,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":6920,"parent_record_offset":6900,"ancestor_record_offset":6716,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":6880,"parent_record_offset":6844,"ancestor_record_offset":6716,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":6900,"parent_record_offset":6880,"ancestor_record_offset":6716,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":6940,"parent_record_offset":6920,"ancestor_record_offset":6716,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::video::uSDL_GetWindowFlags
// 记录形式: compiled_procedure; instances=1
// 源码行: 155
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1008,"record_offset":3940,"rva":16783664,"va":5385492784}
// 编译实例 RVAs: 0x1001930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31129372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385492784,"raw_hex":"e81cffda01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385492863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1008,"record_offset":4060,"parent_record_offset":3940,"ancestor_record_offset":3940,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":4176,"parent_record_offset":4156,"ancestor_record_offset":3940,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1008,"record_offset":4196,"parent_record_offset":4176,"ancestor_record_offset":3940,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1008,"record_offset":4136,"parent_record_offset":4116,"ancestor_record_offset":3940,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":4096,"parent_record_offset":4060,"ancestor_record_offset":3940,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1008,"record_offset":4116,"parent_record_offset":4096,"ancestor_record_offset":3940,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01008.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1008,"record_offset":4156,"parent_record_offset":4136,"ancestor_record_offset":3940,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01008.jsonl.gz"}
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
// 完整结构及原始类型记录: video.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: video.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: video.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: usdl_sys::video::impl$11::eq
// 信息 ID: bf13ec94f03f545d1e321a7a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::handle_sdl_event; module=177, inline_record=21096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::impl$4::default
// 信息 ID: aa6b17abe91e2d4586293613
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::video::VideoSubsystem::display_mode; module=285, inline_record=1916; 不是独立调用边。
// [PDB 内联宿主] usdl::video::VideoSubsystem::current_display_mode; module=285, inline_record=3416; 不是独立调用边。
// [PDB 内联宿主] usdl::video::VideoSubsystem::desktop_display_mode; module=285, inline_record=3912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_CreateWindow
// 信息 ID: 4ded38c8904da350d2b0ba0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::WindowBuilder::build @ 0x6cabad -> 0x1001710 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_DestroyWindow
// 信息 ID: ef7674df5bde652b85e68e80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::impl$11::drop @ 0x6cb793 -> 0x1001770 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_GetCurrentDisplayMode
// 信息 ID: 97d8dfc1ad3b2a7a97ec3707
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::VideoSubsystem::current_display_mode @ 0x6caf61 -> 0x1001c10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_GetCurrentVideoDriver
// 信息 ID: 8d1a6ba2d5ecce2bd7f6d9e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::VideoSubsystem::current_video_driver @ 0x6cafa5 -> 0x1001c70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_GetDesktopDisplayMode
// 信息 ID: ab3764055d0b7f1a054dd927
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::VideoSubsystem::desktop_display_mode @ 0x6cb021 -> 0x1001cc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_GetDisplayBounds
// 信息 ID: e5231bf26e3c7b53e4d09815
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::VideoSubsystem::display_bounds @ 0x6cae78 -> 0x1001a50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_GetDisplayMode
// 信息 ID: b0a19698cebbe6e4a7022aef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::VideoSubsystem::display_mode @ 0x6cad46 -> 0x1001880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_GetDisplayName
// 信息 ID: 93cea1cb4c9563ebd98b6410
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::VideoSubsystem::display_name @ 0x6cad91 -> 0x10018e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_GetNumDisplayModes
// 信息 ID: 051bf778f94f911de8f08c06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::VideoSubsystem::num_display_modes @ 0x6caecb -> 0x1001ab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_GetNumVideoDisplays
// 信息 ID: 5bdb9d65fbd931565f48036d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::VideoSubsystem::num_video_displays @ 0x6caf08 -> 0x1001b00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_GetWindowDisplayIndex
// 信息 ID: 65f69a65c222b4bab8e5dfb7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::Window::display_index @ 0x6cb24b -> 0x1001d20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_GetWindowFlags
// 信息 ID: 5762c62024ced02e426a1c13
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::Window::fullscreen_state @ 0x6cb407 -> 0x1001930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_GetWindowID
// 信息 ID: 6e664d6273e323dc926421e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::Window::id @ 0x6cb4f3 -> 0x10016b0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_GetWindowSize
// 信息 ID: da677e37770601381fdafa77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::Window::size @ 0x6cb531 -> 0x10017c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_MaximizeWindow
// 信息 ID: 8b2166b80a1d0b5a25d3c4e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::Window::maximize @ 0x6cb553 -> 0x1001990 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_MinimizeWindow
// 信息 ID: 61c4026d038d8492011504de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::Window::minimize @ 0x6cb563 -> 0x10019f0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_SetWindowDisplayMode
// 信息 ID: 39017ad03a61ffa3dbd23dc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::Window::set_display_mode @ 0x6cb471 -> 0x1001bb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_SetWindowFullscreen
// 信息 ID: b071a997f12348753f8480c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::Window::set_fullscreen @ 0x6cb3ce -> 0x1001b50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_SetWindowGrab
// 信息 ID: 90d5890e64b38764d4710bac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::Window::set_grab @ 0x6cb576 -> 0x1001820 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_ShowWindow
// 信息 ID: a6d79ed1a810cd677c07469a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::Window::show @ 0x6cb503 -> 0x1001650 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::video::uSDL_Vulkan_GetDrawableSize
// 信息 ID: f619b783d6b8e9cf492595b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::Window::vulkan_drawable_size @ 0x6cb4d1 -> 0x1001d80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
