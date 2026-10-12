// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/semaphore.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 542db24cbc78089506060364bf3c519806c62aa5317136c92c6328ed73224cd5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: semaphore.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 703, 1935, 1981
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::semaphore::create_raw_timeline_semaphore
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B42
// 首条位置: {"module":703,"inlinee":39746}
// 编译实例 RVAs: 0x18c8710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16846760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394695952,"raw_hex":"e8a80f0101","size":5,"soft_fail":false,"target_va":5411542880,"va":5394696115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*184(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5394695952,"raw_hex":"ff90b8000000","size":6,"soft_fail":false,"va":5394696055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::create_semaphore
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1935,"record_offset":21564,"parent_record_offset":21296,"ancestor_record_offset":21296,"inlinee":39751,"source_line":2129,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::SemaphoreCreateInfo::push_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1935,"record_offset":21360,"parent_record_offset":21296,"ancestor_record_offset":21296,"inlinee":39750,"source_line":7639,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::SemaphoreTypeCreateInfo::semaphore_type
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1935,"record_offset":21332,"parent_record_offset":21296,"ancestor_record_offset":21296,"inlinee":39748,"source_line":21895,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1935,"record_offset":21680,"parent_record_offset":21296,"ancestor_record_offset":21296,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1935,"record_offset":21592,"parent_record_offset":21564,"ancestor_record_offset":21296,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1935,"record_offset":21616,"parent_record_offset":21592,"ancestor_record_offset":21296,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1935,"record_offset":21640,"parent_record_offset":21616,"ancestor_record_offset":21296,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":1935,"record_offset":21508,"parent_record_offset":21484,"ancestor_record_offset":21296,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":1935,"record_offset":21412,"parent_record_offset":21388,"ancestor_record_offset":21296,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1935,"record_offset":21436,"parent_record_offset":21412,"ancestor_record_offset":21296,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1935,"record_offset":21484,"parent_record_offset":21460,"ancestor_record_offset":21296,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1935,"record_offset":21388,"parent_record_offset":21360,"ancestor_record_offset":21296,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1935,"record_offset":21460,"parent_record_offset":21436,"ancestor_record_offset":21296,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/01935.jsonl.gz"}
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
// 完整结构及原始类型记录: semaphore.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: semaphore.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: semaphore.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::semaphore::create_raw_timeline_semaphore
// 信息 ID: 92d202d24c1db2c5e340a653
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xbf4f08 -> 0x18c8710 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::device::Queue,core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::device::Queue,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> > > >; module=1935, inline_record=21296; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
