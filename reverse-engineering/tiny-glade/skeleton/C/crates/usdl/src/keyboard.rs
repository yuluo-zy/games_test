// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/usdl/src/keyboard.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5309e764f1b5caf56118bf9552ce329d8236d06ce307f2aa1259a71263ef44ea
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: keyboard.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 177, 283, 286
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use usdl; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.11.0"}]
// use usdl_sys; // namespace candidate
//   package identities: [{"name":"usdl-sys","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: usdl::keyboard::Keycode::from_i32
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":286,"record_offset":1404,"rva":7124704,"va":5375833824}
// 编译实例 RVAs: 0x6cb6e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::keyboard::Scancode::from_i32
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":286,"record_offset":5616,"rva":7124800,"va":5375833920}
// 编译实例 RVAs: 0x6cb740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::keyboard::Keycode::name
// 记录形式: compiled_procedure; instances=1
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":286,"record_offset":452,"rva":7124480,"va":5375833600}
// 编译实例 RVAs: 0x6cb600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_string_lossy
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8959738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833600,"raw_hex":"e8fab68800","size":5,"soft_fail":false,"target_va":5384793392,"va":5375833649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-155074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833600,"raw_hex":"e83ea2fdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375833757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: keyboard.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7111551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833600,"raw_hex":"e8817c93ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375833738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: keyboard.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41223272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833600,"raw_hex":"e868047502","size":5,"soft_fail":false,"target_va":5417057056,"va":5375833779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bf30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: keyboard.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41224459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833600,"raw_hex":"e80b097502","size":5,"soft_fail":false,"target_va":5417058096,"va":5375833632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40787107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833600,"raw_hex":"e8a35c6e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5375833682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::keyboard::uSDL_GetKeyName
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9660587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833600,"raw_hex":"e8ab689300","size":5,"soft_fail":false,"target_va":5385494208,"va":5375833616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":286,"record_offset":740,"parent_record_offset":452,"ancestor_record_offset":452,"inlinee":5422,"source_line":331,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":286,"record_offset":1372,"parent_record_offset":452,"ancestor_record_offset":452,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_direct] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ffi\c_str.rs
//     evidence={"module":286,"record_offset":644,"parent_record_offset":452,"ancestor_record_offset":452,"inlinee":8370,"source_line":253,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::const_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":286,"record_offset":580,"parent_record_offset":452,"ancestor_record_offset":452,"inlinee":19030,"source_line":22,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":286,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":452,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":286,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":452,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":286,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":452,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":286,"record_offset":1164,"parent_record_offset":1016,"ancestor_record_offset":452,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":286,"record_offset":956,"parent_record_offset":928,"ancestor_record_offset":452,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":286,"record_offset":1016,"parent_record_offset":984,"ancestor_record_offset":452,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":286,"record_offset":984,"parent_record_offset":956,"ancestor_record_offset":452,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":286,"record_offset":1052,"parent_record_offset":1016,"ancestor_record_offset":452,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":286,"record_offset":840,"parent_record_offset":804,"ancestor_record_offset":452,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":286,"record_offset":868,"parent_record_offset":840,"ancestor_record_offset":452,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":286,"record_offset":896,"parent_record_offset":868,"ancestor_record_offset":452,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":286,"record_offset":804,"parent_record_offset":776,"ancestor_record_offset":452,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":286,"record_offset":776,"parent_record_offset":740,"ancestor_record_offset":452,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":286,"record_offset":928,"parent_record_offset":896,"ancestor_record_offset":452,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":286,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":452,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ffi\c_str.rs
//     evidence={"module":286,"record_offset":672,"parent_record_offset":644,"ancestor_record_offset":452,"inlinee":8372,"source_line":737,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":286,"record_offset":700,"parent_record_offset":672,"ancestor_record_offset":452,"inlinee":8374,"source_line":2434,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":286,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":452,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":286,"record_offset":1100,"parent_record_offset":1076,"ancestor_record_offset":452,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":286,"record_offset":1292,"parent_record_offset":896,"ancestor_record_offset":452,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::is_null::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":286,"record_offset":608,"parent_record_offset":580,"ancestor_record_offset":452,"inlinee":19032,"source_line":2434,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":286,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":452,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::keyboard::KeyboardUtil::mod_state
// 记录形式: compiled_procedure; instances=1
// 源码行: 338
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":286,"record_offset":260,"rva":7124448,"va":5375833568}
// 编译实例 RVAs: 0x6cb5e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] usdl_sys::keyboard::uSDL_GetModState
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9660727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833568,"raw_hex":"e837699300","size":5,"soft_fail":false,"target_va":5385494304,"va":5375833572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl::keyboard::Mod::from_bits_truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":286,"record_offset":384,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":19060,"source_line":663,"archive":"_pdb/C/modules/00286.jsonl.gz"}
//   [pdb_inline_transitive] usdl::keyboard::_::InternalBitFlags::from_bits_truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":286,"record_offset":412,"parent_record_offset":384,"ancestor_record_offset":260,"inlinee":19061,"source_line":663,"archive":"_pdb/C/modules/00286.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] usdl::keyboard::KeyboardUtil
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl::keyboard::Keycode
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl::keyboard::Mod
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl::keyboard::Scancode
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl::keyboard::_::InternalBitFlags
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
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
// 函数: usdl::keyboard::KeyboardUtil::mod_state
// 信息 ID: 3419e341647b88fabbf5e4ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::keyboard::Keycode::from_i32
// 信息 ID: 3b0275692610bf1ef36ff801
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::event::from_ll @ 0x6ca6d4 -> 0x6cb6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::event::from_ll @ 0x6ca786 -> 0x6cb6e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::keyboard::Keycode::name
// 信息 ID: 9c67363aff1872980ed2e952
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_event_loop::convert_keyboard_input @ 0x516ebb -> 0x6cb600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::keyboard::Scancode::from_i32
// 信息 ID: 87990c8313040b9682ba3b2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::event::from_ll @ 0x6ca6e1 -> 0x6cb740 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::event::from_ll @ 0x6ca793 -> 0x6cb740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
