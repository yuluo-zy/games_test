// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/debug.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 9c229283cf0a8120dfa4ae3b0bec3dec5fec546eb352e13c78cad99c666b591f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: debug.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 703
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::begin_label
// 记录形式: compiled_procedure; instances=1
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":26076,"rva":12519856,"va":5381228976}
// 编译实例 RVAs: 0xbf09b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6147623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381228976,"raw_hex":"e8d931a2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381229122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d797d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1609188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381228976,"raw_hex":"e8e48d1800","size":5,"soft_fail":false,"target_va":5382838224,"va":5381229031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30312890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381228976,"raw_hex":"e8ba89ce01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381229169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30313648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381228976,"raw_hex":"e8b08cce01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381229227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*2200(%rsi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381228976,"raw_hex":"ff9698080000","size":6,"soft_fail":false,"va":5381229095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":703,"record_offset":26292,"parent_record_offset":26076,"ancestor_record_offset":26076,"inlinee":7851,"source_line":257,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::ext::debug_utils::Device::cmd_begin_debug_utils_label
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/ext/debug_utils.rs
//     evidence={"module":703,"record_offset":26764,"parent_record_offset":26076,"ancestor_record_offset":26076,"inlinee":39619,"source_line":29,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$1652::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":26588,"parent_record_offset":26076,"ancestor_record_offset":26076,"inlinee":39618,"source_line":19693,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":26792,"parent_record_offset":26076,"ancestor_record_offset":26076,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":26320,"parent_record_offset":26076,"ancestor_record_offset":26076,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBuffer::raw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":703,"record_offset":26728,"parent_record_offset":26076,"ancestor_record_offset":26076,"inlinee":39590,"source_line":179,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":703,"record_offset":26236,"parent_record_offset":26076,"ancestor_record_offset":26076,"inlinee":39602,"source_line":682,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":26532,"parent_record_offset":26508,"ancestor_record_offset":26076,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":26508,"parent_record_offset":26452,"ancestor_record_offset":26076,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":703,"record_offset":26864,"parent_record_offset":26840,"ancestor_record_offset":26076,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":703,"record_offset":26812,"parent_record_offset":26792,"ancestor_record_offset":26076,"inlinee":7853,"source_line":696,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":26480,"parent_record_offset":26452,"ancestor_record_offset":26076,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":26452,"parent_record_offset":26428,"ancestor_record_offset":26076,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":26428,"parent_record_offset":26404,"ancestor_record_offset":26076,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":26684,"parent_record_offset":26660,"ancestor_record_offset":26076,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":703,"record_offset":26636,"parent_record_offset":26612,"ancestor_record_offset":26076,"inlinee":6026,"source_line":416,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":703,"record_offset":26612,"parent_record_offset":26588,"ancestor_record_offset":26076,"inlinee":6025,"source_line":653,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":26260,"parent_record_offset":26236,"ancestor_record_offset":26076,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":26356,"parent_record_offset":26320,"ancestor_record_offset":26076,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":26660,"parent_record_offset":26636,"ancestor_record_offset":26076,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::end_label
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":27136,"rva":12520256,"va":5381229376}
// 编译实例 RVAs: 0xbf0b40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30312621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229376,"raw_hex":"e8ad88ce01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381229438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::extensions_generated::ext::debug_utils::Device::cmd_end_debug_utils_label
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/ext/debug_utils.rs
//     evidence={"module":703,"record_offset":27356,"parent_record_offset":27136,"ancestor_record_offset":27136,"inlinee":39620,"source_line":39,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBuffer::raw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":703,"record_offset":27324,"parent_record_offset":27136,"ancestor_record_offset":27136,"inlinee":39590,"source_line":179,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":703,"record_offset":27276,"parent_record_offset":27136,"ancestor_record_offset":27136,"inlinee":39602,"source_line":682,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":27296,"parent_record_offset":27276,"ancestor_record_offset":27136,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00703.jsonl.gz"}
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
// 完整结构及原始类型记录: debug.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: debug.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: debug.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::begin_label
// 信息 ID: 1a6c2f358eb7adb5b4237a8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > > @ 0xf87b0 -> 0xbf09b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c26da -> 0xbf09b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$1::record_batch::closure$1<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c3520 -> 0xbf09b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1697f1e -> 0xbf09b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::end_label
// 信息 ID: 3a74e603a13e6f757310415b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > > @ 0xf8adf -> 0xbf0b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c2980 -> 0xbf0b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$1::record_batch::closure$1<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c382f -> 0xbf0b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1699300 -> 0xbf0b40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
