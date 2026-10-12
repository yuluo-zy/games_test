// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/gui/language_menu.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c5e477777f0dee5c5db71e3ebf747c74a8a5fd4df045ad9ef03ba6db81f26174
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: language_menu.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 58, 135
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::language_menu::do_gui_language_menu
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":58,"record_offset":7724,"rva":1379248,"va":5370088368}
// 编译实例 RVAs: 0x150bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=4
//     target_aliases=12 candidates; complete list: language_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-821608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370088368,"raw_hex":"e89876f3ff","size":5,"soft_fail":false,"target_va":5369267664,"va":5370089267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14032d8b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: language_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1951915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370088368,"raw_hex":"e8abc81d00","size":5,"soft_fail":false,"target_va":5372041392,"va":5370089472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bd910
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: language_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4639124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370088368,"raw_hex":"e894c94600","size":5,"soft_fail":false,"target_va":5374728464,"va":5370089335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdf20
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: language_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4640696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370088368,"raw_hex":"e8b8cf4600","size":5,"soft_fail":false,"target_va":5374730016,"va":5370089315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5460510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370088368,"raw_hex":"e81e525300","size":5,"soft_fail":false,"target_va":5375549872,"va":5370089357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5461370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370088368,"raw_hex":"e87a555300","size":5,"soft_fail":false,"target_va":5375550800,"va":5370089425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::gap
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5495024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370088368,"raw_hex":"e8f0d85300","size":5,"soft_fail":false,"target_va":5375584432,"va":5370089403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5495009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370088368,"raw_hex":"e8e1d85300","size":5,"soft_fail":false,"target_va":5375584384,"va":5370089370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":7860,"parent_record_offset":7724,"ancestor_record_offset":7724,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":7908,"parent_record_offset":7884,"ancestor_record_offset":7724,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":7940,"parent_record_offset":7884,"ancestor_record_offset":7724,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":7884,"parent_record_offset":7860,"ancestor_record_offset":7724,"inlinee":7072,"source_line":154,"archive":"_pdb/B/modules/00058.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::language_menu::do_gui_language_menu::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3174
// 首条位置: {"module":135,"inlinee":12660}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":135,"record_offset":4704,"parent_record_offset":4496,"ancestor_record_offset":4496,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":135,"record_offset":5396,"parent_record_offset":4496,"ancestor_record_offset":4496,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":135,"record_offset":4632,"parent_record_offset":4496,"ancestor_record_offset":4496,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":135,"record_offset":4556,"parent_record_offset":4496,"ancestor_record_offset":4496,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":135,"record_offset":5236,"parent_record_offset":5212,"ancestor_record_offset":4496,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":135,"record_offset":5260,"parent_record_offset":5236,"ancestor_record_offset":4496,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":135,"record_offset":5596,"parent_record_offset":5572,"ancestor_record_offset":4496,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":135,"record_offset":5212,"parent_record_offset":5036,"ancestor_record_offset":4496,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":135,"record_offset":5572,"parent_record_offset":5516,"ancestor_record_offset":4496,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":135,"record_offset":4968,"parent_record_offset":4936,"ancestor_record_offset":4496,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":135,"record_offset":5544,"parent_record_offset":5516,"ancestor_record_offset":4496,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":135,"record_offset":5516,"parent_record_offset":5492,"ancestor_record_offset":4496,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":135,"record_offset":5036,"parent_record_offset":5000,"ancestor_record_offset":4496,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":135,"record_offset":5000,"parent_record_offset":4968,"ancestor_record_offset":4496,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":135,"record_offset":5492,"parent_record_offset":5468,"ancestor_record_offset":4496,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":135,"record_offset":5072,"parent_record_offset":5036,"ancestor_record_offset":4496,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":135,"record_offset":4896,"parent_record_offset":4864,"ancestor_record_offset":4496,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":135,"record_offset":4864,"parent_record_offset":4832,"ancestor_record_offset":4496,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":135,"record_offset":4800,"parent_record_offset":4768,"ancestor_record_offset":4496,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":135,"record_offset":4832,"parent_record_offset":4800,"ancestor_record_offset":4496,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":135,"record_offset":4768,"parent_record_offset":4736,"ancestor_record_offset":4496,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":135,"record_offset":4736,"parent_record_offset":4704,"ancestor_record_offset":4496,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":135,"record_offset":4936,"parent_record_offset":4896,"ancestor_record_offset":4496,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":135,"record_offset":5096,"parent_record_offset":5072,"ancestor_record_offset":4496,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":135,"record_offset":5120,"parent_record_offset":5096,"ancestor_record_offset":4496,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":135,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":4496,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":135,"record_offset":5168,"parent_record_offset":5144,"ancestor_record_offset":4496,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":135,"record_offset":5144,"parent_record_offset":5120,"ancestor_record_offset":4496,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":135,"record_offset":5312,"parent_record_offset":4896,"ancestor_record_offset":4496,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":135,"record_offset":4672,"parent_record_offset":4632,"ancestor_record_offset":4496,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00135.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":135,"record_offset":4596,"parent_record_offset":4556,"ancestor_record_offset":4496,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00135.jsonl.gz"}
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
// 完整结构及原始类型记录: language_menu.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: language_menu.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: language_menu.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::gui::language_menu::do_gui_language_menu
// 信息 ID: f07f10b6b464195ef3bf3587
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::language_menu::do_gui_language_menu::closure$0
// 信息 ID: c0334d3e3c0e89fefc5ee3df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::language_menu::do_gui_language_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0 <- toolbar::mei::core::NodeContext::with<tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0>; module=135, inline_record=4496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
