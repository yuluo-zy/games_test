// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/managed/buffer.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a52adcf99b87fc7d35aabce414b859c1b051a2103b8c4a828e01dfc6c5d1097e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: buffer.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 677, 703, 1920, 1921, 1962
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
// 函数符号: rhapsody::raw::managed::buffer::impl$0::dispose
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1921,"record_offset":256,"rva":25885584,"va":5394594704}
// 编译实例 RVAs: 0x18afb90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdf610
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13436636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394594704,"raw_hex":"e824f932ff","size":5,"soft_fail":false,"target_va":5381158416,"va":5394595047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-20239272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394594704,"raw_hex":"e8582ccbfe","size":5,"soft_fail":false,"target_va":5374355568,"va":5394594835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1921,"record_offset":392,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::DisposalBucket::dispose
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1921,"record_offset":504,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":39247,"source_line":143,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1921,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":256,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1921,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":256,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1921,"record_offset":636,"parent_record_offset":612,"ancestor_record_offset":256,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1921,"record_offset":588,"parent_record_offset":552,"ancestor_record_offset":256,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1921,"record_offset":552,"parent_record_offset":528,"ancestor_record_offset":256,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1921,"record_offset":416,"parent_record_offset":392,"ancestor_record_offset":256,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1921,"record_offset":440,"parent_record_offset":416,"ancestor_record_offset":256,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1921,"record_offset":468,"parent_record_offset":416,"ancestor_record_offset":256,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$13::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1921,"record_offset":768,"parent_record_offset":740,"ancestor_record_offset":256,"inlinee":39253,"source_line":346,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$6::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1921,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":256,"inlinee":86328,"source_line":304,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::buffer::impl$0::debug_name
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15116
// 首条位置: {"module":1920,"inlinee":86294}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::buffer::ManagedBuffer::allocation
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9840
// 首条位置: {"module":677,"inlinee":38976}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":677,"record_offset":1216,"parent_record_offset":1188,"ancestor_record_offset":1188,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":677,"record_offset":1244,"parent_record_offset":1188,"ancestor_record_offset":1188,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::buffer::ManagedBuffer::mapped_slice_mut
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x983F
// 首条位置: {"module":677,"inlinee":38975}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":677,"record_offset":1284,"parent_record_offset":1156,"ancestor_record_offset":1156,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::buffer::ManagedBuffer::allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/buffer.rs
//     evidence={"module":677,"record_offset":1188,"parent_record_offset":1156,"ancestor_record_offset":1156,"inlinee":38976,"source_line":39,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":677,"record_offset":1216,"parent_record_offset":1188,"ancestor_record_offset":1156,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":677,"record_offset":1244,"parent_record_offset":1188,"ancestor_record_offset":1156,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::buffer::impl$1::mapped_slice_mut::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/buffer.rs
//     evidence={"module":677,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":1156,"inlinee":38978,"source_line":50,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::buffer::impl$1::mapped_slice_mut::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9842
// 首条位置: {"module":677,"inlinee":38978}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::ManagedResources::create_buffer
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AAE
// 首条位置: {"module":703,"inlinee":39598}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":10872,"parent_record_offset":10848,"ancestor_record_offset":10848,"inlinee":15367,"source_line":298,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$7::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":10908,"parent_record_offset":10872,"ancestor_record_offset":10848,"inlinee":39600,"source_line":379,"archive":"_pdb/B/modules/00703.jsonl.gz"}
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
// 完整结构及原始类型记录: buffer.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::managed::buffer::ManagedBuffer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::buffer::ManagedBuffer>", "constructor": "rhapsody::raw::managed::handle::ManagedHandle", "arguments": ["rhapsody::raw::managed::buffer::ManagedBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::buffer::ManagedBuffer> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::buffer::ManagedBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::handle::GpuOwned<tuple$<>,rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::buffer::ManagedBuffer> >", "constructor": "rhapsody::raw::managed::handle::GpuOwned", "arguments": ["tuple$<>", "rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::buffer::ManagedBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::raw::managed::handle::GpuOwned<tuple$<>,rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::buffer::ManagedBuffer> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::raw::managed::handle::GpuOwned<tuple$<>,rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::buffer::ManagedBuffer> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::managed::handle::GpuOwned<tuple$<>,rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::buffer::ManagedBuffer> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::raw::managed::handle::GpuOwned<tuple$<>,rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::buffer::ManagedBuffer> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<rhapsody::raw::managed::handle::GpuOwned<tuple$<>,rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::buffer::ManagedBuffer> >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["rhapsody::raw::managed::handle::GpuOwned<tuple$<>,rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::buffer::ManagedBuffer> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<rhapsody::raw::managed::handle::GpuOwned<tuple$<>,rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::buffer::ManagedBuffer> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<rhapsody::raw::managed::handle::GpuOwned<tuple$<>,rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::buffer::ManagedBuffer> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::buffer::ManagedBuffer>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore", "arguments": ["rhapsody::raw::managed::buffer::ManagedBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::buffer::ManagedBuffer> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::buffer::ManagedBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::buffer::ManagedBuffer>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::buffer::ManagedBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore::allocate<rhapsody::raw::managed::buffer::ManagedBuffer>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore::allocate", "arguments": ["rhapsody::raw::managed::buffer::ManagedBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::AccessManagedResource::get_maybe_tracked<rhapsody::raw::managed::resources::ManagedResources,rhapsody::raw::managed::buffer::ManagedBuffer>", "constructor": "rhapsody::raw::managed::resources::AccessManagedResource::get_maybe_tracked", "arguments": ["rhapsody::raw::managed::resources::ManagedResources", "rhapsody::raw::managed::buffer::ManagedBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::impl$4::get_tracked<rhapsody::raw::managed::buffer::ManagedBuffer>", "constructor": "rhapsody::raw::managed::resources::impl$4::get_tracked", "arguments": ["rhapsody::raw::managed::buffer::ManagedBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::buffer::ManagedBuffer>", "constructor": "rhapsody::raw::managed::resources::impl$5::dispose", "arguments": ["rhapsody::raw::managed::buffer::ManagedBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::buffer::ManagedBuffer>", "constructor": "rhapsody::raw::managed::resources::ManagedResource", "arguments": ["rhapsody::raw::managed::buffer::ManagedBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "append_only_vec::AppendOnlyVec::push<rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::buffer::ManagedBuffer> >", "constructor": "append_only_vec::AppendOnlyVec::push", "arguments": ["rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::buffer::ManagedBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer>", "constructor": "rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0", "arguments": ["rhapsody::raw::managed::buffer::ManagedBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: buffer.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: buffer.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::buffer::ManagedBuffer::allocation
// 信息 ID: bf5bf9e09e25b12831c7889c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::buffer::Buffer::mapped_slice_mut; module=677, inline_record=1188; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::set_buffer_debug_name; module=703, inline_record=12432; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::log_gpu_crash_info; module=703, inline_record=19772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::buffer::ManagedBuffer::mapped_slice_mut
// 信息 ID: 44043fe0785cf470fe8144cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::buffer::Buffer::mapped_slice_mut; module=677, inline_record=1156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::buffer::impl$0::debug_name
// 信息 ID: a80f80c172219870675cb431
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=1962, inline_record=6720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::buffer::impl$0::dispose
// 信息 ID: de22085bbced073c9cbe500b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::buffer::ManagedBuffer> @ 0x18af887 -> 0x18afb90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::buffer::impl$1::mapped_slice_mut::closure$0
// 信息 ID: 60ba9d111591a515ef7949e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::managed::buffer::impl$1::mapped_slice_mut", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::buffer::Buffer::mapped_slice_mut; module=677, inline_record=1308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::ManagedResources::create_buffer
// 信息 ID: ab556ff838df0af0253e35bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_buffer_impl; module=703, inline_record=10848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
