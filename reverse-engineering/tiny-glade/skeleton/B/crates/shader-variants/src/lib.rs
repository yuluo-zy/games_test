// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/shader-variants/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e7276181be8b5cf88d954854be5e608359d3fdc5013346d4fccfefdc13a8942b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1692
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use shader_pipeline; // namespace candidate
//   package identities: [{"name":"shader-pipeline","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: shader_variants::get_raster_shader_variants
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1692,"record_offset":848,"rva":24348832,"va":5393057952}
// 编译实例 RVAs: 0x17388a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-10210132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393057952,"raw_hex":"e8ac3464ff","size":5,"soft_fail":false,"target_va":5382847888,"va":5393058015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10041577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393057952,"raw_hex":"e8e9389900","size":5,"soft_fail":false,"target_va":5403099664,"va":5393058082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=2
//     target_aliases=26 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18135821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393057952,"raw_hex":"e8f344ebfe","size":5,"soft_fail":false,"target_va":5374923872,"va":5393059688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17976675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393057952,"raw_hex":"e89db2edfe","size":5,"soft_fail":false,"target_va":5375081488,"va":5393058158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17977458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393057952,"raw_hex":"e88eafedfe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393058957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088dec0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15380093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393057952,"raw_hex":"e8835115ff","size":5,"soft_fail":false,"target_va":5377679040,"va":5393059128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-593096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393057952,"raw_hex":"e838f3f6ff","size":5,"soft_fail":false,"target_va":5392465200,"va":5393058291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18482159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393057952,"raw_hex":"e8ef031a01","size":5,"soft_fail":false,"target_va":5411540963,"va":5393058799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1692,"record_offset":1564,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1692,"record_offset":2508,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1692,"record_offset":4664,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::impl$14::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1692,"record_offset":988,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":76883,"source_line":16,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1692,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":848,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1692,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":848,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1692,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":848,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1692,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":848,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1692,"record_offset":1256,"parent_record_offset":1200,"ancestor_record_offset":848,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1692,"record_offset":2584,"parent_record_offset":2552,"ancestor_record_offset":848,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1692,"record_offset":1768,"parent_record_offset":1740,"ancestor_record_offset":848,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1692,"record_offset":1228,"parent_record_offset":1200,"ancestor_record_offset":848,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1692,"record_offset":1200,"parent_record_offset":1176,"ancestor_record_offset":848,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1692,"record_offset":2632,"parent_record_offset":2608,"ancestor_record_offset":848,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1692,"record_offset":2608,"parent_record_offset":2584,"ancestor_record_offset":848,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1692,"record_offset":1828,"parent_record_offset":1796,"ancestor_record_offset":848,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1692,"record_offset":1796,"parent_record_offset":1768,"ancestor_record_offset":848,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1692,"record_offset":1176,"parent_record_offset":1152,"ancestor_record_offset":848,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1692,"record_offset":1704,"parent_record_offset":1676,"ancestor_record_offset":848,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1692,"record_offset":1676,"parent_record_offset":1648,"ancestor_record_offset":848,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1692,"record_offset":1620,"parent_record_offset":1592,"ancestor_record_offset":848,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1692,"record_offset":1648,"parent_record_offset":1620,"ancestor_record_offset":848,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1692,"record_offset":1592,"parent_record_offset":1564,"ancestor_record_offset":848,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1692,"record_offset":2552,"parent_record_offset":2508,"ancestor_record_offset":848,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1692,"record_offset":1740,"parent_record_offset":1704,"ancestor_record_offset":848,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1692,"record_offset":1980,"parent_record_offset":1956,"ancestor_record_offset":848,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1692,"record_offset":1036,"parent_record_offset":988,"ancestor_record_offset":848,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1692,"record_offset":1956,"parent_record_offset":1704,"ancestor_record_offset":848,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1692,"record_offset":1104,"parent_record_offset":988,"ancestor_record_offset":848,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1692,"record_offset":2672,"parent_record_offset":2508,"ancestor_record_offset":848,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::shader_defines::impl$1::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/shader_defines.rs
//     evidence={"module":1692,"record_offset":1076,"parent_record_offset":988,"ancestor_record_offset":848,"inlinee":76885,"source_line":3,"archive":"_pdb/B/modules/01692.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: shader_variants::get_raster_shader_variants
// 信息 ID: 2065fb22ef7bdbc311ec5bd2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::shader::compile_shader_variants @ 0x9383e6 -> 0x17388a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
