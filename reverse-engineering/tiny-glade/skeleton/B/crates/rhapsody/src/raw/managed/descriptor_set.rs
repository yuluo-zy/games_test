// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/managed/descriptor_set.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 29fcac2048d8a9e5ef4fa33f7d5ca0677600208041f76b2d7bd79ff9fb7970b6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: descriptor_set.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 703, 1920, 1962
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::descriptor_set::impl$1::dispose
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15119
// 首条位置: {"module":1920,"inlinee":86297}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::disposal::DisposalBucket::dispose
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1920,"record_offset":5404,"parent_record_offset":5380,"ancestor_record_offset":5380,"inlinee":39247,"source_line":143,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1920,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":5380,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1920,"record_offset":5560,"parent_record_offset":5536,"ancestor_record_offset":5380,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1920,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":5380,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1920,"record_offset":5488,"parent_record_offset":5452,"ancestor_record_offset":5380,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1920,"record_offset":5452,"parent_record_offset":5428,"ancestor_record_offset":5380,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1920,"record_offset":5600,"parent_record_offset":5452,"ancestor_record_offset":5380,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$11::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1920,"record_offset":5428,"parent_record_offset":5404,"ancestor_record_offset":5380,"inlinee":86299,"source_line":334,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::ManagedResources::create_persistent_descriptor_set
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B61
// 首条位置: {"module":703,"inlinee":39777}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":144844,"parent_record_offset":144820,"ancestor_record_offset":144820,"inlinee":15367,"source_line":298,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$11::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":144876,"parent_record_offset":144844,"ancestor_record_offset":144820,"inlinee":39779,"source_line":419,"archive":"_pdb/B/modules/00703.jsonl.gz"}
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
// 完整结构及原始类型记录: descriptor_set.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore", "arguments": ["rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore::allocate<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore::allocate", "arguments": ["rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::impl$4::get_tracked<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>", "constructor": "rhapsody::raw::managed::resources::impl$4::get_tracked", "arguments": ["rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>", "constructor": "rhapsody::raw::managed::resources::impl$5::dispose", "arguments": ["rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>", "constructor": "rhapsody::raw::managed::resources::ManagedResource", "arguments": ["rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "append_only_vec::AppendOnlyVec::push<rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> >", "constructor": "append_only_vec::AppendOnlyVec::push", "arguments": ["rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>", "constructor": "rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0", "arguments": ["rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: descriptor_set.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: descriptor_set.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::descriptor_set::impl$1::dispose
// 信息 ID: 2e9b90f9cc0dc05008c96f98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>; module=1920, inline_record=5380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::ManagedResources::create_persistent_descriptor_set
// 信息 ID: 0097ee85e977b77fc25e36d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set; module=703, inline_record=144820; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
