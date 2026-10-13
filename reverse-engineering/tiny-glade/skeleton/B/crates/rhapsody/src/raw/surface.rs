// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/surface.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6efdf5ec1f41b138adae6d4e5345619d066671d3d15e55142591a405509a6bd1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: surface.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 5
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use ash_window; // namespace candidate
//   package identities: [{"name":"ash-window","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.13.0"}]
// use core; // namespace candidate
//   package identities: []
// use raw_window_handle; // namespace candidate
//   package identities: [{"name":"raw-window-handle","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.6.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::surface::Surface::create<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":5,"record_offset":16124,"rva":202208,"va":5368911328}
// 编译实例 RVAs: 0x315e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$winit..window..Window$u20$as$u20$raw_window_handle..borrowed..HasDisplayHandle$GT$::display_handle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6712599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368911328,"raw_hex":"e8176d6600","size":5,"soft_fail":false,"target_va":5375623968,"va":5368911364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..window..Window$u20$as$u20$raw_window_handle..borrowed..HasWindowHandle$GT$::window_handle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6712478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368911328,"raw_hex":"e89e6c6600","size":5,"soft_fail":false,"target_va":5375623904,"va":5368911421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::extensions_generated::khr::surface::Instance::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12935206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368911328,"raw_hex":"e82660c500","size":5,"soft_fail":false,"target_va":5381846752,"va":5368911541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash_window::create_surface
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ash-window","version":"0.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12444048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368911328,"raw_hex":"e890e1bd00","size":5,"soft_fail":false,"target_va":5381355552,"va":5368911499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42631246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368911328,"raw_hex":"e84e808a02","size":5,"soft_fail":false,"target_va":5411542880,"va":5368911629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":5,"record_offset":16544,"parent_record_offset":16124,"ancestor_record_offset":16124,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00005.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":5,"record_offset":16756,"parent_record_offset":16124,"ancestor_record_offset":16124,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00005.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":5,"record_offset":16792,"parent_record_offset":16124,"ancestor_record_offset":16124,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00005.jsonl.gz"}
//   [pdb_inline_direct] raw_window_handle::borrowed::impl$0::display_handle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-window-handle","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-window-handle-0.6.1/src/borrowed.rs
//     evidence={"module":5,"record_offset":16404,"parent_record_offset":16124,"ancestor_record_offset":16124,"inlinee":5341,"source_line":39,"archive":"_pdb/B/modules/00005.jsonl.gz"}
//   [pdb_inline_direct] raw_window_handle::borrowed::impl$11::window_handle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-window-handle","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-window-handle-0.6.1/src/borrowed.rs
//     evidence={"module":5,"record_offset":16580,"parent_record_offset":16124,"ancestor_record_offset":16124,"inlinee":5348,"source_line":167,"archive":"_pdb/B/modules/00005.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":5,"record_offset":16476,"parent_record_offset":16452,"ancestor_record_offset":16124,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00005.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":5,"record_offset":16452,"parent_record_offset":16428,"ancestor_record_offset":16124,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00005.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":5,"record_offset":16500,"parent_record_offset":16476,"ancestor_record_offset":16124,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00005.jsonl.gz"}
//   [pdb_inline_transitive] raw_window_handle::borrowed::impl$15::window_handle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-window-handle","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-window-handle-0.6.1/src/borrowed.rs
//     evidence={"module":5,"record_offset":16604,"parent_record_offset":16580,"ancestor_record_offset":16124,"inlinee":5350,"source_line":197,"archive":"_pdb/B/modules/00005.jsonl.gz"}
//   [pdb_inline_transitive] raw_window_handle::borrowed::impl$4::display_handle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-window-handle","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-window-handle-0.6.1/src/borrowed.rs
//     evidence={"module":5,"record_offset":16428,"parent_record_offset":16404,"ancestor_record_offset":16124,"inlinee":5343,"source_line":69,"archive":"_pdb/B/modules/00005.jsonl.gz"}
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
// 完整结构及原始类型记录: surface.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::surface::Surface
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::surface::Surface::create<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >", "constructor": "rhapsody::raw::surface::Surface::create", "arguments": ["ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >", "ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<rhapsody::raw::surface::Surface,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["rhapsody::raw::surface::Surface", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: surface.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: surface.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::surface::Surface::create<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >
// 信息 ID: f291fec05022d26e7bf89e77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::sync::Arc -> ["winit::window::Window", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::sync::Arc<winit::window::Window,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] alloc::sync::Arc -> ["winit::window::Window", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::sync::Arc<winit::window::Window,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::surface::Surface::create -> ["ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >", "ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::init::GraphicsDevicesForSurface::new<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > @ 0x340cc9 -> 0x315e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
