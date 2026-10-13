// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/save/snapshot_gc.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 4676ef360851d4648a9a6ba881ed474aece46e802fa04c220781d58bf514621b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: snapshot_gc.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1814
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot_gc::parse_save_file_snapshots<std::io::buffered::bufreader::BufReader<std::fs::File> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1814,"record_offset":1716,"rva":29444576,"va":5398153696}
// 编译实例 RVAs: 0x1c149e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-22475173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398153696,"raw_hex":"e85b0ea9fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5398153872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097cb70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot_gc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19496859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398153696,"raw_hex":"e86580d6fe","size":5,"soft_fail":false,"target_va":5378657136,"va":5398153990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a257e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot_gc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18805373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398153696,"raw_hex":"e8830de1fe","size":5,"soft_fail":false,"target_va":5379348448,"va":5398153816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*28007289(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5398153696,"raw_hex":"ff15795bab01","size":6,"soft_fail":false,"va":5398153881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1814,"record_offset":2020,"parent_record_offset":1716,"ancestor_record_offset":1716,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01814.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1814,"record_offset":1988,"parent_record_offset":1716,"ancestor_record_offset":1716,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01814.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::from_reader
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1814,"record_offset":1932,"parent_record_offset":1716,"ancestor_record_offset":1716,"inlinee":81553,"source_line":82,"archive":"_pdb/C/modules/01814.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1814,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":1716,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01814.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1814,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":1716,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01814.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1814,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":1716,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01814.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1814,"record_offset":2224,"parent_record_offset":2196,"ancestor_record_offset":1716,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01814.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1814,"record_offset":2196,"parent_record_offset":2168,"ancestor_record_offset":1716,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01814.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1814,"record_offset":2624,"parent_record_offset":2596,"ancestor_record_offset":1716,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01814.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1814,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":1716,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01814.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1814,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":1716,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01814.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1814,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":1716,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01814.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1814,"record_offset":1956,"parent_record_offset":1932,"ancestor_record_offset":1716,"inlinee":7612,"source_line":59,"archive":"_pdb/C/modules/01814.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":1814,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":1716,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/01814.jsonl.gz"}
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
// 完整结构及原始类型记录: snapshot_gc.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::snapshot_gc::HistoryKey
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::impl$1::deserialize<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >,country_core::systems::save::snapshot_gc::HistoryKey,ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > > >", "constructor": "utils::selective_deserialize::impl$1::deserialize", "arguments": ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >", "country_core::systems::save::snapshot_gc::HistoryKey", "ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >", "constructor": "utils::selective_deserialize::SelectiveDeserialize", "arguments": ["country_core::systems::save::snapshot_gc::HistoryKey", "utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "utils::selective_deserialize::PartialVisitor", "arguments": ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > > >", "constructor": "serde_json::de::impl$7::deserialize_struct", "arguments": ["serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >", "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::snapshot_gc::SnapshotsKey
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >", "constructor": "utils::selective_deserialize::SelectiveDeserialize", "arguments": ["country_core::systems::save::snapshot_gc::SnapshotsKey", "alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::impl$1::deserialize<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >,country_core::systems::save::snapshot_gc::HistoryKey,ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > > >", "constructor": "utils::selective_deserialize::impl$1::deserialize", "arguments": ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >", "country_core::systems::save::snapshot_gc::HistoryKey", "ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::impl$1::deserialize<alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global>,country_core::systems::save::snapshot_gc::SnapshotsKey,ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > > >", "constructor": "utils::selective_deserialize::impl$1::deserialize", "arguments": ["alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global>", "country_core::systems::save::snapshot_gc::SnapshotsKey", "ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >", "constructor": "utils::selective_deserialize::PartialVisitor", "arguments": ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impl$1::fmt<utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "serde_core::de::impl$1::fmt", "arguments": ["utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >", "constructor": "utils::selective_deserialize::SelectiveDeserialize", "arguments": ["country_core::systems::save::snapshot_gc::HistoryKey", "utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "utils::selective_deserialize::PartialVisitor", "arguments": ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > > >", "constructor": "serde_json::de::impl$7::deserialize_struct", "arguments": ["serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >", "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "serde_json::de::impl$7::deserialize_struct", "arguments": ["serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >", "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: snapshot_gc.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: snapshot_gc.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot_gc::parse_save_file_snapshots<std::io::buffered::bufreader::BufReader<std::fs::File> >
// 信息 ID: 6fa8cd1c72ccdc5a3b3c2b40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] std::io::buffered::bufreader::BufReader -> ["std::fs::File"]; 字段布局 = unknown。
// [类型关系] country_core::systems::save::snapshot_gc::parse_save_file_snapshots -> ["std::io::buffered::bufreader::BufReader<std::fs::File>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_thread::snapshot_gc_thread @ 0x1b7a288 -> 0x1c149e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
