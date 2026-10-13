// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/usdl/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: fca61609eae2a591318bd7d8b3abc4707b55c83f14e5612dac6c0ba559c2d188
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 285, 1013, 2528
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use usdl; // namespace candidate
//   package identities: [{"name":"usdl","source":"local_or_path","version":"0.1.0"}]
// use usdl_sys; // namespace candidate
//   package identities: [{"name":"usdl-sys","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: usdl::video::WindowBuilder::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":380,"rva":7121712,"va":5375830832}
// 编译实例 RVAs: 0x6cab30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-152412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e8a4acfdff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375831111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f574e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8964485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e885c98800","size":5,"soft_fail":false,"target_va":5384795360,"va":5375830870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40791089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e8316c6e02","size":5,"soft_fail":false,"target_va":5416622240,"va":5375831146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::sdl::get_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e83df8ffff","size":5,"soft_fail":false,"target_va":5375829072,"va":5375831054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_CreateWindow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e85e6b9300","size":5,"soft_fail":false,"target_va":5385492240,"va":5375830957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\ffi\c_str.rs
//     evidence={"module":285,"record_offset":520,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":6244,"source_line":257,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":285,"record_offset":580,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":285,"record_offset":896,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":285,"record_offset":548,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":285,"record_offset":840,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] usdl::video::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":644,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":19082,"source_line":132,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":380,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":380,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":380,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":980,"parent_record_offset":952,"ancestor_record_offset":380,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":285,"record_offset":952,"parent_record_offset":920,"ancestor_record_offset":380,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\ffi\c_str.rs
//     evidence={"module":285,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":380,"inlinee":4646,"source_line":700,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":285,"record_offset":1156,"parent_record_offset":1104,"ancestor_record_offset":380,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":285,"record_offset":688,"parent_record_offset":644,"ancestor_record_offset":380,"inlinee":4151,"source_line":3134,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":285,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":380,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] usdl::sdl::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/sdl.rs
//     evidence={"module":285,"record_offset":744,"parent_record_offset":644,"ancestor_record_offset":380,"inlinee":19019,"source_line":78,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] usdl::video::impl$5::build::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":612,"parent_record_offset":580,"ancestor_record_offset":380,"inlinee":19084,"source_line":296,"archive":"_pdb/C/modules/00285.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: usdl::video::WindowBuilder::build
// 信息 ID: 7921b0ef0ee911cb2740e598
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8cbc -> 0x6cab30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
