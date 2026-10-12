// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/utils/debug_viz.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2c1bc4ec176f48cbfaa05b162a43dd6d50825a35caeb4cfa740591892c88ebc4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: debug_viz.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 391, 416, 421, 445, 453, 461, 486, 1302, 1338, 1390, 1448, 2831
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$6::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":53152,"rva":15494896,"va":5384204016}
// 编译实例 RVAs: 0xec6ef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field5_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6619158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384204016,"raw_hex":"e8eaff9aff","size":5,"soft_fail":false,"target_va":5377585120,"va":5384204273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugSphere::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6594
// 首条位置: {"module":453,"inlinee":26004}
// 编译实例 RVAs: 0xec3880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugSphere::color
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6596
// 首条位置: {"module":453,"inlinee":26006}
// 编译实例 RVAs: 0xec38b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugSphere::radius
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x56F4
// 首条位置: {"module":391,"inlinee":22260}
// 编译实例 RVAs: 0xec38d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugSphere::solid
// 记录形式: compiled_procedure; instances=1
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":19032,"rva":15481072,"va":5384190192}
// 编译实例 RVAs: 0xec38f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugSphere::line_width
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6595
// 首条位置: {"module":453,"inlinee":26005}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugSphere::show
// 记录形式: compiled_procedure; instances=1
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":19160,"rva":15481104,"va":5384190224}
// 编译实例 RVAs: 0xec3910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce150
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12540489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190224,"raw_hex":"e8b7a540ff","size":5,"soft_fail":false,"target_va":5371650384,"va":5384190868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11440734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190224,"raw_hex":"e8a26d51ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384190425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11440919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190224,"raw_hex":"e8e96c51ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384190626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405caa20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9408624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190224,"raw_hex":"e8906f70ff","size":5,"soft_fail":false,"target_va":5374781984,"va":5384190603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19402784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190224,"raw_hex":"e820102801","size":5,"soft_fail":false,"target_va":5403593360,"va":5384190571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18787470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190224,"raw_hex":"e88eac1e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5384190669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6660507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190224,"raw_hex":"e8655e9aff","size":5,"soft_fail":false,"target_va":5377529888,"va":5384190390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6663565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190224,"raw_hex":"e873529aff","size":5,"soft_fail":false,"target_va":5377527120,"va":5384190680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1338,"record_offset":19312,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1338,"record_offset":19392,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":20368,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":19608,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":22967,"source_line":102,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":1338,"record_offset":20340,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":22969,"source_line":24,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":19504,"parent_record_offset":19480,"ancestor_record_offset":19160,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":19528,"parent_record_offset":19504,"ancestor_record_offset":19160,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":20520,"parent_record_offset":20496,"ancestor_record_offset":19160,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":19448,"parent_record_offset":19420,"ancestor_record_offset":19160,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":19480,"parent_record_offset":19448,"ancestor_record_offset":19160,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":20496,"parent_record_offset":20440,"ancestor_record_offset":19160,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1338,"record_offset":19420,"parent_record_offset":19392,"ancestor_record_offset":19160,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1338,"record_offset":19360,"parent_record_offset":19336,"ancestor_record_offset":19160,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":20468,"parent_record_offset":20440,"ancestor_record_offset":19160,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":19904,"parent_record_offset":19876,"ancestor_record_offset":19160,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":19876,"parent_record_offset":19848,"ancestor_record_offset":19160,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":20060,"parent_record_offset":19876,"ancestor_record_offset":19160,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":20440,"parent_record_offset":20416,"ancestor_record_offset":19160,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":19812,"parent_record_offset":19784,"ancestor_record_offset":19160,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":19784,"parent_record_offset":19756,"ancestor_record_offset":19160,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":19728,"parent_record_offset":19700,"ancestor_record_offset":19160,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":19756,"parent_record_offset":19728,"ancestor_record_offset":19160,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":19700,"parent_record_offset":19672,"ancestor_record_offset":19160,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1338,"record_offset":19672,"parent_record_offset":19644,"ancestor_record_offset":19160,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1338,"record_offset":19644,"parent_record_offset":19608,"ancestor_record_offset":19160,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1338,"record_offset":20208,"parent_record_offset":20184,"ancestor_record_offset":19160,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":19848,"parent_record_offset":19812,"ancestor_record_offset":19160,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1338,"record_offset":20124,"parent_record_offset":20100,"ancestor_record_offset":19160,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1338,"record_offset":19336,"parent_record_offset":19312,"ancestor_record_offset":19160,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1338,"record_offset":20100,"parent_record_offset":19812,"ancestor_record_offset":19160,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":20184,"parent_record_offset":19608,"ancestor_record_offset":19160,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":19556,"parent_record_offset":19528,"ancestor_record_offset":19160,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1338,"record_offset":20244,"parent_record_offset":20208,"ancestor_record_offset":19160,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1338,"record_offset":20268,"parent_record_offset":20244,"ancestor_record_offset":19160,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1338,"record_offset":20300,"parent_record_offset":20208,"ancestor_record_offset":19160,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$1::show::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":20776,"rva":15481792,"va":5384190912}
// 编译实例 RVAs: 0xec3bc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11441578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e8566a51ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384191269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8851818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e896ee78ff","size":5,"soft_fail":false,"target_va":5375340432,"va":5384192245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-883030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e8aa86f2ff","size":5,"soft_fail":false,"target_va":5383308144,"va":5384191169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19401830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e8660c2801","size":5,"soft_fail":false,"target_va":5403593360,"va":5384191525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t18786079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e81fa71e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5384192060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18727218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e832c11d01","size":5,"soft_fail":false,"target_va":5402919072,"va":5384191849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6664969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e8f74c9aff","size":5,"soft_fail":false,"target_va":5377527120,"va":5384192084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6945341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e8c30596ff","size":5,"soft_fail":false,"target_va":5377246528,"va":5384191864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18786570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e80aa91e01","size":5,"soft_fail":false,"target_va":5402978688,"va":5384192113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e83cf2ffff","size":5,"soft_fail":false,"target_va":5384188496,"va":5384192015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18788905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e829b21e01","size":5,"soft_fail":false,"target_va":5402980128,"va":5384191218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18789304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e8b8b31e01","size":5,"soft_fail":false,"target_va":5402980656,"va":5384191347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8834897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e8af3079ff","size":5,"soft_fail":false,"target_va":5375356896,"va":5384191788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7834553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e8477488ff","size":5,"soft_fail":false,"target_va":5376356912,"va":5384191460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7830594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e8be8388ff","size":5,"soft_fail":false,"target_va":5376360560,"va":5384191149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7834830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384190912,"raw_hex":"e8327388ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5384190969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":24824,"parent_record_offset":20776,"ancestor_record_offset":20776,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":24976,"parent_record_offset":20776,"ancestor_record_offset":20776,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":24268,"parent_record_offset":20776,"ancestor_record_offset":20776,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1338,"record_offset":23008,"parent_record_offset":20776,"ancestor_record_offset":20776,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":23664,"parent_record_offset":20776,"ancestor_record_offset":20776,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":24240,"parent_record_offset":20776,"ancestor_record_offset":20776,"inlinee":26724,"source_line":189,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1338,"record_offset":21960,"parent_record_offset":20776,"ancestor_record_offset":20776,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1338,"record_offset":20944,"parent_record_offset":20776,"ancestor_record_offset":20776,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":23440,"parent_record_offset":23416,"ancestor_record_offset":20776,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":23464,"parent_record_offset":23440,"ancestor_record_offset":20776,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":23416,"parent_record_offset":23372,"ancestor_record_offset":20776,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":24496,"parent_record_offset":24468,"ancestor_record_offset":20776,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":24468,"parent_record_offset":24436,"ancestor_record_offset":20776,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":24436,"parent_record_offset":24408,"ancestor_record_offset":20776,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":23372,"parent_record_offset":23336,"ancestor_record_offset":20776,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":23336,"parent_record_offset":23300,"ancestor_record_offset":20776,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":23536,"parent_record_offset":23336,"ancestor_record_offset":20776,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":23260,"parent_record_offset":23224,"ancestor_record_offset":20776,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":25000,"parent_record_offset":24976,"ancestor_record_offset":20776,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":23224,"parent_record_offset":23188,"ancestor_record_offset":20776,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":23152,"parent_record_offset":23116,"ancestor_record_offset":20776,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":23188,"parent_record_offset":23152,"ancestor_record_offset":20776,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":23116,"parent_record_offset":23080,"ancestor_record_offset":20776,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1338,"record_offset":23080,"parent_record_offset":23044,"ancestor_record_offset":20776,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1338,"record_offset":23044,"parent_record_offset":23008,"ancestor_record_offset":20776,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":25024,"parent_record_offset":25000,"ancestor_record_offset":20776,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":24548,"parent_record_offset":24380,"ancestor_record_offset":20776,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":24408,"parent_record_offset":24380,"ancestor_record_offset":20776,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":23300,"parent_record_offset":23260,"ancestor_record_offset":20776,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":24316,"parent_record_offset":24292,"ancestor_record_offset":20776,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":24344,"parent_record_offset":24316,"ancestor_record_offset":20776,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":24292,"parent_record_offset":24268,"ancestor_record_offset":20776,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1338,"record_offset":24580,"parent_record_offset":24344,"ancestor_record_offset":20776,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1338,"record_offset":24380,"parent_record_offset":24344,"ancestor_record_offset":20776,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1338,"record_offset":22360,"parent_record_offset":22332,"ancestor_record_offset":20776,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1338,"record_offset":22412,"parent_record_offset":22388,"ancestor_record_offset":20776,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1338,"record_offset":22388,"parent_record_offset":22360,"ancestor_record_offset":20776,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1338,"record_offset":22300,"parent_record_offset":22272,"ancestor_record_offset":20776,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1338,"record_offset":22332,"parent_record_offset":22300,"ancestor_record_offset":20776,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1338,"record_offset":22504,"parent_record_offset":22332,"ancestor_record_offset":20776,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1338,"record_offset":22616,"parent_record_offset":22592,"ancestor_record_offset":20776,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1338,"record_offset":23600,"parent_record_offset":23576,"ancestor_record_offset":20776,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1338,"record_offset":22436,"parent_record_offset":22412,"ancestor_record_offset":20776,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1338,"record_offset":21824,"parent_record_offset":21732,"ancestor_record_offset":20776,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1338,"record_offset":22196,"parent_record_offset":22168,"ancestor_record_offset":20776,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1338,"record_offset":21540,"parent_record_offset":21080,"ancestor_record_offset":20776,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1338,"record_offset":22244,"parent_record_offset":22068,"ancestor_record_offset":20776,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1338,"record_offset":21908,"parent_record_offset":21880,"ancestor_record_offset":20776,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1338,"record_offset":23576,"parent_record_offset":23260,"ancestor_record_offset":20776,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":22568,"parent_record_offset":22272,"ancestor_record_offset":20776,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1338,"record_offset":21316,"parent_record_offset":21292,"ancestor_record_offset":20776,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1338,"record_offset":21292,"parent_record_offset":21268,"ancestor_record_offset":20776,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1338,"record_offset":21500,"parent_record_offset":21468,"ancestor_record_offset":20776,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":23492,"parent_record_offset":23464,"ancestor_record_offset":20776,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":22460,"parent_record_offset":22436,"ancestor_record_offset":20776,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1338,"record_offset":22968,"parent_record_offset":22032,"ancestor_record_offset":20776,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1338,"record_offset":21768,"parent_record_offset":21732,"ancestor_record_offset":20776,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1338,"record_offset":21792,"parent_record_offset":21768,"ancestor_record_offset":20776,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1338,"record_offset":21268,"parent_record_offset":21244,"ancestor_record_offset":20776,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1338,"record_offset":21596,"parent_record_offset":21572,"ancestor_record_offset":20776,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1338,"record_offset":21244,"parent_record_offset":21136,"ancestor_record_offset":20776,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1338,"record_offset":21732,"parent_record_offset":21700,"ancestor_record_offset":20776,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1338,"record_offset":20976,"parent_record_offset":20944,"ancestor_record_offset":20776,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1338,"record_offset":21108,"parent_record_offset":21080,"ancestor_record_offset":20776,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1338,"record_offset":21080,"parent_record_offset":21048,"ancestor_record_offset":20776,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1338,"record_offset":21024,"parent_record_offset":21000,"ancestor_record_offset":20776,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1338,"record_offset":21048,"parent_record_offset":21024,"ancestor_record_offset":20776,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1338,"record_offset":21000,"parent_record_offset":20976,"ancestor_record_offset":20776,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1338,"record_offset":21572,"parent_record_offset":21048,"ancestor_record_offset":20776,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1338,"record_offset":21700,"parent_record_offset":21668,"ancestor_record_offset":20776,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1338,"record_offset":21856,"parent_record_offset":21700,"ancestor_record_offset":20776,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1338,"record_offset":21668,"parent_record_offset":21644,"ancestor_record_offset":20776,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1338,"record_offset":21644,"parent_record_offset":20944,"ancestor_record_offset":20776,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1338,"record_offset":21880,"parent_record_offset":21856,"ancestor_record_offset":20776,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1338,"record_offset":21136,"parent_record_offset":21108,"ancestor_record_offset":20776,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1338,"record_offset":21384,"parent_record_offset":21136,"ancestor_record_offset":20776,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1338,"record_offset":21356,"parent_record_offset":21136,"ancestor_record_offset":20776,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1338,"record_offset":21188,"parent_record_offset":21136,"ancestor_record_offset":20776,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1338,"record_offset":21996,"parent_record_offset":21960,"ancestor_record_offset":20776,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1338,"record_offset":22108,"parent_record_offset":22068,"ancestor_record_offset":20776,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1338,"record_offset":22272,"parent_record_offset":22068,"ancestor_record_offset":20776,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1338,"record_offset":22540,"parent_record_offset":22272,"ancestor_record_offset":20776,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1338,"record_offset":22136,"parent_record_offset":22108,"ancestor_record_offset":20776,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1338,"record_offset":22168,"parent_record_offset":22136,"ancestor_record_offset":20776,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1338,"record_offset":22068,"parent_record_offset":22032,"ancestor_record_offset":20776,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1338,"record_offset":22032,"parent_record_offset":21996,"ancestor_record_offset":20776,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$7::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":53272,"rva":15495168,"va":5384204288}
// 编译实例 RVAs: 0xec7000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6618565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384204288,"raw_hex":"e83b029bff","size":5,"soft_fail":false,"target_va":5377585888,"va":5384204448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-6620436","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5384204288,"raw_hex":"e9ecfa9aff","size":5,"soft_fail":false,"target_va":5377583920,"va":5384204351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugColor::as_array
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF34F
// 首条位置: {"module":1338,"inlinee":62287}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle3d::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":25064,"rva":15483232,"va":5384192352}
// 编译实例 RVAs: 0xec4160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle3d::color
// 记录形式: compiled_procedure; instances=1
// 源码行: 192
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":25196,"rva":15483280,"va":5384192400}
// 编译实例 RVAs: 0xec4190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle3d::line_width
// 记录形式: compiled_procedure; instances=1
// 源码行: 198
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":25328,"rva":15483328,"va":5384192448}
// 编译实例 RVAs: 0xec41c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle3d::show
// 记录形式: compiled_procedure; instances=1
// 源码行: 208
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":25464,"rva":15483376,"va":5384192496}
// 编译实例 RVAs: 0xec41f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce150
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12543936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384192496,"raw_hex":"e8409840ff","size":5,"soft_fail":false,"target_va":5371650384,"va":5384194315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11443274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384192496,"raw_hex":"e8b66351ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384192965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11443030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384192496,"raw_hex":"e8aa6451ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384192737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405caa20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9411212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384192496,"raw_hex":"e8746570ff","size":5,"soft_fail":false,"target_va":5374781984,"va":5384193191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7249302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384192496,"raw_hex":"e86a6291ff","size":5,"soft_fail":false,"target_va":5376944000,"va":5384193297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19400207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384192496,"raw_hex":"e80f062801","size":5,"soft_fail":false,"target_va":5403593360,"va":5384193148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18784170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384192496,"raw_hex":"e8aa9f1e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5384193969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6663017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384192496,"raw_hex":"e897549aff","size":5,"soft_fail":false,"target_va":5377529888,"va":5384192900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6666878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384192496,"raw_hex":"e882459aff","size":5,"soft_fail":false,"target_va":5377527120,"va":5384193993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve3
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6238032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384192496,"raw_hex":"e8b0d0a0ff","size":5,"soft_fail":false,"target_va":5377954656,"va":5384192683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6237640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384192496,"raw_hex":"e838d2a0ff","size":5,"soft_fail":false,"target_va":5377955632,"va":5384193267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6237345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384192496,"raw_hex":"e85fd3a0ff","size":5,"soft_fail":false,"target_va":5377955424,"va":5384192764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1338,"record_offset":26060,"parent_record_offset":25464,"ancestor_record_offset":25464,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1338,"record_offset":26144,"parent_record_offset":25464,"ancestor_record_offset":25464,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1338,"record_offset":25632,"parent_record_offset":25464,"ancestor_record_offset":25464,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1338,"record_offset":27600,"parent_record_offset":25464,"ancestor_record_offset":25464,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":25840,"parent_record_offset":25464,"ancestor_record_offset":25464,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":26640,"parent_record_offset":25464,"ancestor_record_offset":25464,"inlinee":22967,"source_line":102,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":1338,"record_offset":27372,"parent_record_offset":25464,"ancestor_record_offset":25464,"inlinee":22969,"source_line":24,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1338,"record_offset":27992,"parent_record_offset":25464,"ancestor_record_offset":25464,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":26272,"parent_record_offset":26248,"ancestor_record_offset":25464,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":26296,"parent_record_offset":26272,"ancestor_record_offset":25464,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":26016,"parent_record_offset":25992,"ancestor_record_offset":25464,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":26216,"parent_record_offset":26180,"ancestor_record_offset":25464,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":26248,"parent_record_offset":26216,"ancestor_record_offset":25464,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":25992,"parent_record_offset":25892,"ancestor_record_offset":25464,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1338,"record_offset":26180,"parent_record_offset":26144,"ancestor_record_offset":25464,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1338,"record_offset":26108,"parent_record_offset":26084,"ancestor_record_offset":25464,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":25924,"parent_record_offset":25892,"ancestor_record_offset":25464,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":26936,"parent_record_offset":26908,"ancestor_record_offset":25464,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":26908,"parent_record_offset":26880,"ancestor_record_offset":25464,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":27092,"parent_record_offset":26908,"ancestor_record_offset":25464,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":25892,"parent_record_offset":25864,"ancestor_record_offset":25464,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":26844,"parent_record_offset":26816,"ancestor_record_offset":25464,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":26816,"parent_record_offset":26788,"ancestor_record_offset":25464,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":26760,"parent_record_offset":26732,"ancestor_record_offset":25464,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":26788,"parent_record_offset":26760,"ancestor_record_offset":25464,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":26732,"parent_record_offset":26704,"ancestor_record_offset":25464,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1338,"record_offset":26704,"parent_record_offset":26676,"ancestor_record_offset":25464,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1338,"record_offset":26676,"parent_record_offset":26640,"ancestor_record_offset":25464,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1338,"record_offset":27240,"parent_record_offset":27216,"ancestor_record_offset":25464,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":26880,"parent_record_offset":26844,"ancestor_record_offset":25464,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1338,"record_offset":25720,"parent_record_offset":25656,"ancestor_record_offset":25464,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1338,"record_offset":27648,"parent_record_offset":27624,"ancestor_record_offset":25464,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1338,"record_offset":27156,"parent_record_offset":27132,"ancestor_record_offset":25464,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1338,"record_offset":27624,"parent_record_offset":27600,"ancestor_record_offset":25464,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1338,"record_offset":25744,"parent_record_offset":25720,"ancestor_record_offset":25464,"inlinee":6097,"source_line":697,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1338,"record_offset":25956,"parent_record_offset":25924,"ancestor_record_offset":25464,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1338,"record_offset":28064,"parent_record_offset":28016,"ancestor_record_offset":25464,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1338,"record_offset":25692,"parent_record_offset":25656,"ancestor_record_offset":25464,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1338,"record_offset":25656,"parent_record_offset":25632,"ancestor_record_offset":25464,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1338,"record_offset":26084,"parent_record_offset":26060,"ancestor_record_offset":25464,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1338,"record_offset":27132,"parent_record_offset":26844,"ancestor_record_offset":25464,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1338,"record_offset":25768,"parent_record_offset":25744,"ancestor_record_offset":25464,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":25864,"parent_record_offset":25840,"ancestor_record_offset":25464,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":25792,"parent_record_offset":25768,"ancestor_record_offset":25464,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":26324,"parent_record_offset":26296,"ancestor_record_offset":25464,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1338,"record_offset":27276,"parent_record_offset":27240,"ancestor_record_offset":25464,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1338,"record_offset":27300,"parent_record_offset":27276,"ancestor_record_offset":25464,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1338,"record_offset":27332,"parent_record_offset":27240,"ancestor_record_offset":25464,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1338,"record_offset":28092,"parent_record_offset":28016,"ancestor_record_offset":25464,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1338,"record_offset":28016,"parent_record_offset":27992,"ancestor_record_offset":25464,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1338,"record_offset":28132,"parent_record_offset":28092,"ancestor_record_offset":25464,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$3::show::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":29800,"rva":15485984,"va":5384195104}
// 编译实例 RVAs: 0xec4c20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11445529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384195104,"raw_hex":"e8e75a51ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384195220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11446066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384195104,"raw_hex":"e8ce5851ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384195773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8856250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384195104,"raw_hex":"e846dd78ff","size":5,"soft_fail":false,"target_va":5375340432,"va":5384196677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8228282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384195104,"raw_hex":"e8467282ff","size":5,"soft_fail":false,"target_va":5375968464,"va":5384196741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d72c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1384745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384195104,"raw_hex":"e8d7deeaff","size":5,"soft_fail":false,"target_va":5382810688,"va":5384195428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19398088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384195104,"raw_hex":"e8c8fd2701","size":5,"soft_fail":false,"target_va":5403593360,"va":5384195267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18782901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384195104,"raw_hex":"e8b59a1e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5384195238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18722669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384195104,"raw_hex":"e86daf1d01","size":5,"soft_fail":false,"target_va":5402919072,"va":5384196398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6669403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384195104,"raw_hex":"e8a53b9aff","size":5,"soft_fail":false,"target_va":5377527120,"va":5384196518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6949612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384195104,"raw_hex":"e814f595ff","size":5,"soft_fail":false,"target_va":5377246528,"va":5384196135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8839063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384195104,"raw_hex":"e8692079ff","size":5,"soft_fail":false,"target_va":5375356896,"va":5384195954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":33892,"parent_record_offset":29800,"ancestor_record_offset":29800,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":34044,"parent_record_offset":29800,"ancestor_record_offset":29800,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":29972,"parent_record_offset":29800,"ancestor_record_offset":29800,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":31908,"parent_record_offset":29800,"ancestor_record_offset":29800,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":34128,"parent_record_offset":29800,"ancestor_record_offset":29800,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":31880,"parent_record_offset":29800,"ancestor_record_offset":29800,"inlinee":23597,"source_line":201,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::custom_shader
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":31108,"parent_record_offset":29800,"ancestor_record_offset":29800,"inlinee":22974,"source_line":174,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":30528,"parent_record_offset":29800,"ancestor_record_offset":29800,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugColor::as_array
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1338,"record_offset":30484,"parent_record_offset":29800,"ancestor_record_offset":29800,"inlinee":62287,"source_line":162,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":30272,"parent_record_offset":30248,"ancestor_record_offset":29800,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":30296,"parent_record_offset":30272,"ancestor_record_offset":29800,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":31828,"parent_record_offset":31804,"ancestor_record_offset":29800,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":30248,"parent_record_offset":30148,"ancestor_record_offset":29800,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":31804,"parent_record_offset":31780,"ancestor_record_offset":29800,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":32220,"parent_record_offset":32192,"ancestor_record_offset":29800,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":34204,"parent_record_offset":34176,"ancestor_record_offset":29800,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":32192,"parent_record_offset":32156,"ancestor_record_offset":29800,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":32324,"parent_record_offset":32300,"ancestor_record_offset":29800,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":32156,"parent_record_offset":32124,"ancestor_record_offset":29800,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":30148,"parent_record_offset":30120,"ancestor_record_offset":29800,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":30120,"parent_record_offset":30092,"ancestor_record_offset":29800,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":30368,"parent_record_offset":30120,"ancestor_record_offset":29800,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":31780,"parent_record_offset":31756,"ancestor_record_offset":29800,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":30056,"parent_record_offset":30028,"ancestor_record_offset":29800,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":34068,"parent_record_offset":34044,"ancestor_record_offset":29800,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":30028,"parent_record_offset":30000,"ancestor_record_offset":29800,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":30660,"parent_record_offset":30632,"ancestor_record_offset":29800,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":30000,"parent_record_offset":29972,"ancestor_record_offset":29800,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":30632,"parent_record_offset":30604,"ancestor_record_offset":29800,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1338,"record_offset":30604,"parent_record_offset":30576,"ancestor_record_offset":29800,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1338,"record_offset":30576,"parent_record_offset":30528,"ancestor_record_offset":29800,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":32300,"parent_record_offset":32068,"ancestor_record_offset":29800,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":34092,"parent_record_offset":34068,"ancestor_record_offset":29800,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":32272,"parent_record_offset":32068,"ancestor_record_offset":29800,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":32124,"parent_record_offset":32068,"ancestor_record_offset":29800,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":32452,"parent_record_offset":32068,"ancestor_record_offset":29800,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":30092,"parent_record_offset":30056,"ancestor_record_offset":29800,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":31980,"parent_record_offset":31944,"ancestor_record_offset":29800,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":32020,"parent_record_offset":31980,"ancestor_record_offset":29800,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":31944,"parent_record_offset":31908,"ancestor_record_offset":29800,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1338,"record_offset":32484,"parent_record_offset":32020,"ancestor_record_offset":29800,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1338,"record_offset":32068,"parent_record_offset":32020,"ancestor_record_offset":29800,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1338,"record_offset":30192,"parent_record_offset":30148,"ancestor_record_offset":29800,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1338,"record_offset":30216,"parent_record_offset":30192,"ancestor_record_offset":29800,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1338,"record_offset":30436,"parent_record_offset":30412,"ancestor_record_offset":29800,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1338,"record_offset":34232,"parent_record_offset":34204,"ancestor_record_offset":29800,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1338,"record_offset":30412,"parent_record_offset":30056,"ancestor_record_offset":29800,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":31708,"parent_record_offset":31108,"ancestor_record_offset":29800,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1338,"record_offset":32356,"parent_record_offset":32068,"ancestor_record_offset":29800,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":30324,"parent_record_offset":30296,"ancestor_record_offset":29800,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":32384,"parent_record_offset":32356,"ancestor_record_offset":29800,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$3::show::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 268
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":34368,"rva":15487648,"va":5384196768}
// 编译实例 RVAs: 0xec52a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11447151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384196768,"raw_hex":"e8915451ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384196842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11447682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384196768,"raw_hex":"e87e5251ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384197389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8857850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384196768,"raw_hex":"e806d778ff","size":5,"soft_fail":false,"target_va":5375340432,"va":5384198277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8229882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384196768,"raw_hex":"e8066c82ff","size":5,"soft_fail":false,"target_va":5375968464,"va":5384198341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d72c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1386357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384196768,"raw_hex":"e88bd8eaff","size":5,"soft_fail":false,"target_va":5382810688,"va":5384197040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19395985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384196768,"raw_hex":"e891f52701","size":5,"soft_fail":false,"target_va":5403593360,"va":5384197370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18780042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384196768,"raw_hex":"e88a8f1e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5384198097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18721053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384196768,"raw_hex":"e81da91d01","size":5,"soft_fail":false,"target_va":5402919072,"va":5384198014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6951231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384196768,"raw_hex":"e8c1ee95ff","size":5,"soft_fail":false,"target_va":5377246528,"va":5384197754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8840682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384196768,"raw_hex":"e8161a79ff","size":5,"soft_fail":false,"target_va":5375356896,"va":5384197573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":34536,"parent_record_offset":34368,"ancestor_record_offset":34368,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":38244,"parent_record_offset":34368,"ancestor_record_offset":34368,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":36108,"parent_record_offset":34368,"ancestor_record_offset":34368,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":38328,"parent_record_offset":34368,"ancestor_record_offset":34368,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":36080,"parent_record_offset":34368,"ancestor_record_offset":34368,"inlinee":23597,"source_line":201,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::custom_shader
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":35308,"parent_record_offset":34368,"ancestor_record_offset":34368,"inlinee":22974,"source_line":174,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":34728,"parent_record_offset":34368,"ancestor_record_offset":34368,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugColor::as_array
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1338,"record_offset":34684,"parent_record_offset":34368,"ancestor_record_offset":34368,"inlinee":62287,"source_line":162,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":34592,"parent_record_offset":34568,"ancestor_record_offset":34368,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":34616,"parent_record_offset":34592,"ancestor_record_offset":34368,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":36028,"parent_record_offset":36004,"ancestor_record_offset":34368,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":34568,"parent_record_offset":34536,"ancestor_record_offset":34368,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":36004,"parent_record_offset":35980,"ancestor_record_offset":34368,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":36420,"parent_record_offset":36392,"ancestor_record_offset":34368,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":36392,"parent_record_offset":36356,"ancestor_record_offset":34368,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":36524,"parent_record_offset":36500,"ancestor_record_offset":34368,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":36356,"parent_record_offset":36324,"ancestor_record_offset":34368,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":35036,"parent_record_offset":35008,"ancestor_record_offset":34368,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":35008,"parent_record_offset":34980,"ancestor_record_offset":34368,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":35184,"parent_record_offset":35008,"ancestor_record_offset":34368,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":35980,"parent_record_offset":35956,"ancestor_record_offset":34368,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":34944,"parent_record_offset":34916,"ancestor_record_offset":34368,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":38268,"parent_record_offset":38244,"ancestor_record_offset":34368,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":34916,"parent_record_offset":34888,"ancestor_record_offset":34368,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":34860,"parent_record_offset":34832,"ancestor_record_offset":34368,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":34888,"parent_record_offset":34860,"ancestor_record_offset":34368,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":34832,"parent_record_offset":34804,"ancestor_record_offset":34368,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1338,"record_offset":34804,"parent_record_offset":34776,"ancestor_record_offset":34368,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1338,"record_offset":34776,"parent_record_offset":34728,"ancestor_record_offset":34368,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":36500,"parent_record_offset":36268,"ancestor_record_offset":34368,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":38292,"parent_record_offset":38268,"ancestor_record_offset":34368,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":36472,"parent_record_offset":36268,"ancestor_record_offset":34368,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":36324,"parent_record_offset":36268,"ancestor_record_offset":34368,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":36652,"parent_record_offset":36268,"ancestor_record_offset":34368,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":34980,"parent_record_offset":34944,"ancestor_record_offset":34368,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":36180,"parent_record_offset":36144,"ancestor_record_offset":34368,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":36220,"parent_record_offset":36180,"ancestor_record_offset":34368,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":36144,"parent_record_offset":36108,"ancestor_record_offset":34368,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1338,"record_offset":36684,"parent_record_offset":36220,"ancestor_record_offset":34368,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1338,"record_offset":36268,"parent_record_offset":36220,"ancestor_record_offset":34368,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1338,"record_offset":35244,"parent_record_offset":35220,"ancestor_record_offset":34368,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1338,"record_offset":35220,"parent_record_offset":34944,"ancestor_record_offset":34368,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":35908,"parent_record_offset":35308,"ancestor_record_offset":34368,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1338,"record_offset":36556,"parent_record_offset":36268,"ancestor_record_offset":34368,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":34640,"parent_record_offset":34616,"ancestor_record_offset":34368,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":36584,"parent_record_offset":36556,"ancestor_record_offset":34368,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle2d::new
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 309
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CE1
// 首条位置: {"module":416,"inlinee":23777}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle2d::color
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 318
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CE9
// 首条位置: {"module":416,"inlinee":23785}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle2d::show
// 记录形式: compiled_procedure; instances=1
// 源码行: 329
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":38500,"rva":15489248,"va":5384198368}
// 编译实例 RVAs: 0xec58e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce150
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12548898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384198368,"raw_hex":"e8de8440ff","size":5,"soft_fail":false,"target_va":5371650384,"va":5384199277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11448951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384198368,"raw_hex":"e8894d51ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384198642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11448772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384198368,"raw_hex":"e83c4e51ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384198479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405caa20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9416854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384198368,"raw_hex":"e86a4f70ff","size":5,"soft_fail":false,"target_va":5374781984,"va":5384198833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19394554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384198368,"raw_hex":"e8faef2701","size":5,"soft_fail":false,"target_va":5403593360,"va":5384198801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18779245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384198368,"raw_hex":"e86d8c1e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5384198894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6668717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384198368,"raw_hex":"e8533e9aff","size":5,"soft_fail":false,"target_va":5377529888,"va":5384198600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6671790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384198368,"raw_hex":"e852329aff","size":5,"soft_fail":false,"target_va":5377527120,"va":5384198905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve3
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6243780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384198368,"raw_hex":"e83cbaa0ff","size":5,"soft_fail":false,"target_va":5377954656,"va":5384198431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1338,"record_offset":38860,"parent_record_offset":38500,"ancestor_record_offset":38500,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1338,"record_offset":38944,"parent_record_offset":38500,"ancestor_record_offset":38500,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":38656,"parent_record_offset":38500,"ancestor_record_offset":38500,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":39440,"parent_record_offset":38500,"ancestor_record_offset":38500,"inlinee":22967,"source_line":102,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":1338,"record_offset":40172,"parent_record_offset":38500,"ancestor_record_offset":38500,"inlinee":22969,"source_line":24,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":39072,"parent_record_offset":39048,"ancestor_record_offset":38500,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":39096,"parent_record_offset":39072,"ancestor_record_offset":38500,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":38816,"parent_record_offset":38792,"ancestor_record_offset":38500,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":39016,"parent_record_offset":38980,"ancestor_record_offset":38500,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":39048,"parent_record_offset":39016,"ancestor_record_offset":38500,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":38792,"parent_record_offset":38700,"ancestor_record_offset":38500,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1338,"record_offset":38980,"parent_record_offset":38944,"ancestor_record_offset":38500,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1338,"record_offset":38908,"parent_record_offset":38884,"ancestor_record_offset":38500,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":38728,"parent_record_offset":38700,"ancestor_record_offset":38500,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":39736,"parent_record_offset":39708,"ancestor_record_offset":38500,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":39708,"parent_record_offset":39680,"ancestor_record_offset":38500,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":39892,"parent_record_offset":39708,"ancestor_record_offset":38500,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":38700,"parent_record_offset":38676,"ancestor_record_offset":38500,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":39644,"parent_record_offset":39616,"ancestor_record_offset":38500,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":39616,"parent_record_offset":39588,"ancestor_record_offset":38500,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":39560,"parent_record_offset":39532,"ancestor_record_offset":38500,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":39588,"parent_record_offset":39560,"ancestor_record_offset":38500,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":39532,"parent_record_offset":39504,"ancestor_record_offset":38500,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1338,"record_offset":39504,"parent_record_offset":39476,"ancestor_record_offset":38500,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1338,"record_offset":39476,"parent_record_offset":39440,"ancestor_record_offset":38500,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1338,"record_offset":40040,"parent_record_offset":40016,"ancestor_record_offset":38500,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":39680,"parent_record_offset":39644,"ancestor_record_offset":38500,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1338,"record_offset":39956,"parent_record_offset":39932,"ancestor_record_offset":38500,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1338,"record_offset":38756,"parent_record_offset":38728,"ancestor_record_offset":38500,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1338,"record_offset":38884,"parent_record_offset":38860,"ancestor_record_offset":38500,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1338,"record_offset":39932,"parent_record_offset":39644,"ancestor_record_offset":38500,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":38676,"parent_record_offset":38656,"ancestor_record_offset":38500,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":39124,"parent_record_offset":39096,"ancestor_record_offset":38500,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1338,"record_offset":40076,"parent_record_offset":40040,"ancestor_record_offset":38500,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1338,"record_offset":40100,"parent_record_offset":40076,"ancestor_record_offset":38500,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1338,"record_offset":40132,"parent_record_offset":40040,"ancestor_record_offset":38500,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$4::show::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 338
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":40808,"rva":15490192,"va":5384199312}
// 编译实例 RVAs: 0xec5c90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11449695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384199312,"raw_hex":"e8a14a51ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384199386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11450501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384199312,"raw_hex":"e87b4751ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384200208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8859946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384199312,"raw_hex":"e8d6ce78ff","size":5,"soft_fail":false,"target_va":5375340432,"va":5384200373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d72c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1388990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384199312,"raw_hex":"e842ceeaff","size":5,"soft_fail":false,"target_va":5382810688,"va":5384199673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19393836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384199312,"raw_hex":"e82ced2701","size":5,"soft_fail":false,"target_va":5403593360,"va":5384199519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t18778651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384199312,"raw_hex":"e81b8a1e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5384199488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18719105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384199312,"raw_hex":"e881a11d01","size":5,"soft_fail":false,"target_va":5402919072,"va":5384199962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6673034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384199312,"raw_hex":"e8762d9aff","size":5,"soft_fail":false,"target_va":5377527120,"va":5384200149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6953454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384199312,"raw_hex":"e812e695ff","size":5,"soft_fail":false,"target_va":5377246528,"va":5384199977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8843020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384199312,"raw_hex":"e8f41079ff","size":5,"soft_fail":false,"target_va":5375356896,"va":5384199911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":40976,"parent_record_offset":40808,"ancestor_record_offset":40808,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":42864,"parent_record_offset":40808,"ancestor_record_offset":40808,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":41124,"parent_record_offset":40808,"ancestor_record_offset":40808,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":42308,"parent_record_offset":40808,"ancestor_record_offset":40808,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":42948,"parent_record_offset":40808,"ancestor_record_offset":40808,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":42280,"parent_record_offset":40808,"ancestor_record_offset":40808,"inlinee":23597,"source_line":201,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":41704,"parent_record_offset":40808,"ancestor_record_offset":40808,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugColor::as_array
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1338,"record_offset":41660,"parent_record_offset":40808,"ancestor_record_offset":40808,"inlinee":62287,"source_line":162,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":41032,"parent_record_offset":41008,"ancestor_record_offset":40808,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":41056,"parent_record_offset":41032,"ancestor_record_offset":40808,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":43020,"parent_record_offset":42996,"ancestor_record_offset":40808,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":41008,"parent_record_offset":40976,"ancestor_record_offset":40808,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":42996,"parent_record_offset":42972,"ancestor_record_offset":40808,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1338,"record_offset":42972,"parent_record_offset":42948,"ancestor_record_offset":40808,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":42536,"parent_record_offset":42508,"ancestor_record_offset":40808,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":43136,"parent_record_offset":43108,"ancestor_record_offset":40808,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":42508,"parent_record_offset":42476,"ancestor_record_offset":40808,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":42476,"parent_record_offset":42448,"ancestor_record_offset":40808,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":41324,"parent_record_offset":41292,"ancestor_record_offset":40808,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":41292,"parent_record_offset":41260,"ancestor_record_offset":40808,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":41544,"parent_record_offset":41292,"ancestor_record_offset":40808,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":43108,"parent_record_offset":43084,"ancestor_record_offset":40808,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":41220,"parent_record_offset":41188,"ancestor_record_offset":40808,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":42888,"parent_record_offset":42864,"ancestor_record_offset":40808,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":41188,"parent_record_offset":41156,"ancestor_record_offset":40808,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":41832,"parent_record_offset":41804,"ancestor_record_offset":40808,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":41156,"parent_record_offset":41124,"ancestor_record_offset":40808,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":41804,"parent_record_offset":41776,"ancestor_record_offset":40808,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1338,"record_offset":41776,"parent_record_offset":41748,"ancestor_record_offset":40808,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1338,"record_offset":41748,"parent_record_offset":41704,"ancestor_record_offset":40808,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":42912,"parent_record_offset":42888,"ancestor_record_offset":40808,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":42588,"parent_record_offset":42420,"ancestor_record_offset":40808,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":42448,"parent_record_offset":42420,"ancestor_record_offset":40808,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":41260,"parent_record_offset":41220,"ancestor_record_offset":40808,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":42356,"parent_record_offset":42332,"ancestor_record_offset":40808,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":42384,"parent_record_offset":42356,"ancestor_record_offset":40808,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":42332,"parent_record_offset":42308,"ancestor_record_offset":40808,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1338,"record_offset":42620,"parent_record_offset":42384,"ancestor_record_offset":40808,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1338,"record_offset":42420,"parent_record_offset":42384,"ancestor_record_offset":40808,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1338,"record_offset":41368,"parent_record_offset":41324,"ancestor_record_offset":40808,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1338,"record_offset":41392,"parent_record_offset":41368,"ancestor_record_offset":40808,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1338,"record_offset":41612,"parent_record_offset":41588,"ancestor_record_offset":40808,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1338,"record_offset":43164,"parent_record_offset":43136,"ancestor_record_offset":40808,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1338,"record_offset":41588,"parent_record_offset":41220,"ancestor_record_offset":40808,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":43084,"parent_record_offset":43060,"ancestor_record_offset":40808,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":41080,"parent_record_offset":41056,"ancestor_record_offset":40808,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugLine::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 366
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CE4
// 首条位置: {"module":416,"inlinee":23780}
// 编译实例 RVAs: 0xec60d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugLine::color
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=10
// 源码行: 384
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CE3
// 首条位置: {"module":416,"inlinee":23779}
// 编译实例 RVAs: 0xec6110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugLine::set_ui_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 389
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":43548,"rva":15491392,"va":5384200512}
// 编译实例 RVAs: 0xec6140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugLine::dashed
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=10
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x56F6
// 首条位置: {"module":391,"inlinee":22262}
// 编译实例 RVAs: 0xec6170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugLine::line_width
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=10
// 源码行: 400
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F5A
// 首条位置: {"module":421,"inlinee":24410}
// 编译实例 RVAs: 0xec61a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugLine::show
// 记录形式: compiled_procedure; instances=1
// 源码行: 405
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":43940,"rva":15491536,"va":5384200656}
// 编译实例 RVAs: 0xec61d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce150
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12551546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384200656,"raw_hex":"e8867a40ff","size":5,"soft_fail":false,"target_va":5371650384,"va":5384201925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11451691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384200656,"raw_hex":"e8d54251ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384201382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11451870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384200656,"raw_hex":"e8224251ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384201577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405caa20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9419574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384200656,"raw_hex":"e8ca4470ff","size":5,"soft_fail":false,"target_va":5374781984,"va":5384201553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406db760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8302344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384200656,"raw_hex":"e8f85081ff","size":5,"soft_fail":false,"target_va":5375899488,"va":5384201827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceda30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1935777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384200656,"raw_hex":"e85f76e2ff","size":5,"soft_fail":false,"target_va":5382265392,"va":5384201164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d72c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1390393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384200656,"raw_hex":"e8c7c8eaff","size":5,"soft_fail":false,"target_va":5382810688,"va":5384201076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19391842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384200656,"raw_hex":"e862e52701","size":5,"soft_fail":false,"target_va":5403593360,"va":5384201513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18776522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384200656,"raw_hex":"e8ca811e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5384201617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6670952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384200656,"raw_hex":"e898359aff","size":5,"soft_fail":false,"target_va":5377529888,"va":5384200835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6674513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384200656,"raw_hex":"e8af279aff","size":5,"soft_fail":false,"target_va":5377527120,"va":5384201628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1338,"record_offset":44140,"parent_record_offset":43940,"ancestor_record_offset":43940,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1338,"record_offset":45056,"parent_record_offset":43940,"ancestor_record_offset":43940,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1338,"record_offset":44112,"parent_record_offset":43940,"ancestor_record_offset":43940,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1338,"record_offset":44220,"parent_record_offset":43940,"ancestor_record_offset":43940,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":46048,"parent_record_offset":43940,"ancestor_record_offset":43940,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1338,"record_offset":44340,"parent_record_offset":43940,"ancestor_record_offset":43940,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":45288,"parent_record_offset":43940,"ancestor_record_offset":43940,"inlinee":22967,"source_line":102,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":1338,"record_offset":46020,"parent_record_offset":43940,"ancestor_record_offset":43940,"inlinee":22969,"source_line":24,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1338,"record_offset":44848,"parent_record_offset":43940,"ancestor_record_offset":43940,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1338,"record_offset":44424,"parent_record_offset":43940,"ancestor_record_offset":43940,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1338,"record_offset":44668,"parent_record_offset":43940,"ancestor_record_offset":43940,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":45184,"parent_record_offset":45160,"ancestor_record_offset":43940,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":45208,"parent_record_offset":45184,"ancestor_record_offset":43940,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":46200,"parent_record_offset":46176,"ancestor_record_offset":43940,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":45128,"parent_record_offset":45092,"ancestor_record_offset":43940,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":45160,"parent_record_offset":45128,"ancestor_record_offset":43940,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":46176,"parent_record_offset":46120,"ancestor_record_offset":43940,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1338,"record_offset":45092,"parent_record_offset":45056,"ancestor_record_offset":43940,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1338,"record_offset":44188,"parent_record_offset":44164,"ancestor_record_offset":43940,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":46148,"parent_record_offset":46120,"ancestor_record_offset":43940,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":45584,"parent_record_offset":45556,"ancestor_record_offset":43940,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":45556,"parent_record_offset":45528,"ancestor_record_offset":43940,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":45740,"parent_record_offset":45556,"ancestor_record_offset":43940,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":46120,"parent_record_offset":46096,"ancestor_record_offset":43940,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":45492,"parent_record_offset":45464,"ancestor_record_offset":43940,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":45464,"parent_record_offset":45436,"ancestor_record_offset":43940,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":45408,"parent_record_offset":45380,"ancestor_record_offset":43940,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":45436,"parent_record_offset":45408,"ancestor_record_offset":43940,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":45380,"parent_record_offset":45352,"ancestor_record_offset":43940,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1338,"record_offset":45352,"parent_record_offset":45324,"ancestor_record_offset":43940,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1338,"record_offset":45324,"parent_record_offset":45288,"ancestor_record_offset":43940,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1338,"record_offset":45888,"parent_record_offset":45864,"ancestor_record_offset":43940,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":45528,"parent_record_offset":45492,"ancestor_record_offset":43940,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1338,"record_offset":45804,"parent_record_offset":45780,"ancestor_record_offset":43940,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1338,"record_offset":44628,"parent_record_offset":44604,"ancestor_record_offset":43940,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1338,"record_offset":44604,"parent_record_offset":44448,"ancestor_record_offset":43940,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1338,"record_offset":46348,"parent_record_offset":46324,"ancestor_record_offset":43940,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1338,"record_offset":44920,"parent_record_offset":44876,"ancestor_record_offset":43940,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1338,"record_offset":44164,"parent_record_offset":44140,"ancestor_record_offset":43940,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1338,"record_offset":45780,"parent_record_offset":45492,"ancestor_record_offset":43940,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":45864,"parent_record_offset":45288,"ancestor_record_offset":43940,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1338,"record_offset":44388,"parent_record_offset":44364,"ancestor_record_offset":43940,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1338,"record_offset":44568,"parent_record_offset":44544,"ancestor_record_offset":43940,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1338,"record_offset":44516,"parent_record_offset":44480,"ancestor_record_offset":43940,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":45236,"parent_record_offset":45208,"ancestor_record_offset":43940,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1338,"record_offset":44364,"parent_record_offset":44340,"ancestor_record_offset":43940,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1338,"record_offset":44544,"parent_record_offset":44480,"ancestor_record_offset":43940,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1338,"record_offset":44480,"parent_record_offset":44448,"ancestor_record_offset":43940,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1338,"record_offset":45924,"parent_record_offset":45888,"ancestor_record_offset":43940,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1338,"record_offset":45948,"parent_record_offset":45924,"ancestor_record_offset":43940,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1338,"record_offset":45980,"parent_record_offset":45888,"ancestor_record_offset":43940,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::debug_viz::DebugColor::as_array
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1338,"record_offset":44284,"parent_record_offset":44256,"ancestor_record_offset":43940,"inlinee":62287,"source_line":162,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::debug_viz::impl$5::show::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1338,"record_offset":44256,"parent_record_offset":44220,"ancestor_record_offset":43940,"inlinee":62289,"source_line":420,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1338,"record_offset":44980,"parent_record_offset":44876,"ancestor_record_offset":43940,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1338,"record_offset":44876,"parent_record_offset":44848,"ancestor_record_offset":43940,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1338,"record_offset":44448,"parent_record_offset":44424,"ancestor_record_offset":43940,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1338,"record_offset":45016,"parent_record_offset":44980,"ancestor_record_offset":43940,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$5::show::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 420
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF351
// 首条位置: {"module":1338,"inlinee":62289}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugColor::as_array
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1338,"record_offset":44284,"parent_record_offset":44256,"ancestor_record_offset":44256,"inlinee":62287,"source_line":162,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$5::show::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 432
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1338,"record_offset":47352,"rva":15493120,"va":5384202240}
// 编译实例 RVAs: 0xec6800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11452686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384202240,"raw_hex":"e8f23e51ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384202377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11453138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384202240,"raw_hex":"e82e3d51ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384202845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8863434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384202240,"raw_hex":"e836c178ff","size":5,"soft_fail":false,"target_va":5375340432,"va":5384203861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec4d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8235466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384202240,"raw_hex":"e8365682ff","size":5,"soft_fail":false,"target_va":5375968464,"va":5384203925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19390931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384202240,"raw_hex":"e8d3e12701","size":5,"soft_fail":false,"target_va":5403593360,"va":5384202424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t18775744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384202240,"raw_hex":"e8c07e1e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5384202395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18715537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384202240,"raw_hex":"e891931d01","size":5,"soft_fail":false,"target_va":5402919072,"va":5384203530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6676523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384202240,"raw_hex":"e8d51f9aff","size":5,"soft_fail":false,"target_va":5377527120,"va":5384203638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6956754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384202240,"raw_hex":"e82ed995ff","size":5,"soft_fail":false,"target_va":5377246528,"va":5384203277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-8846032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384202240,"raw_hex":"e8300579ff","size":5,"soft_fail":false,"target_va":5375356896,"va":5384202923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":52704,"parent_record_offset":47352,"ancestor_record_offset":47352,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":52856,"parent_record_offset":47352,"ancestor_record_offset":47352,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":47524,"parent_record_offset":47352,"ancestor_record_offset":47352,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":49940,"parent_record_offset":47352,"ancestor_record_offset":47352,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":52940,"parent_record_offset":47352,"ancestor_record_offset":47352,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":49168,"parent_record_offset":47352,"ancestor_record_offset":47352,"inlinee":23597,"source_line":201,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::custom_shader
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":49196,"parent_record_offset":47352,"ancestor_record_offset":47352,"inlinee":22974,"source_line":174,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":48560,"parent_record_offset":47352,"ancestor_record_offset":47352,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::set_ui_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1338,"record_offset":49140,"parent_record_offset":47352,"ancestor_record_offset":47352,"inlinee":22975,"source_line":163,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":47848,"parent_record_offset":47824,"ancestor_record_offset":47352,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":47872,"parent_record_offset":47848,"ancestor_record_offset":47352,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":49888,"parent_record_offset":49864,"ancestor_record_offset":47352,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":47824,"parent_record_offset":47724,"ancestor_record_offset":47352,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1338,"record_offset":49864,"parent_record_offset":49840,"ancestor_record_offset":47352,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":50256,"parent_record_offset":50228,"ancestor_record_offset":47352,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":53016,"parent_record_offset":52988,"ancestor_record_offset":47352,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":50228,"parent_record_offset":50192,"ancestor_record_offset":47352,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":50360,"parent_record_offset":50336,"ancestor_record_offset":47352,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":50192,"parent_record_offset":50160,"ancestor_record_offset":47352,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":47724,"parent_record_offset":47692,"ancestor_record_offset":47352,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":47692,"parent_record_offset":47660,"ancestor_record_offset":47352,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":47944,"parent_record_offset":47692,"ancestor_record_offset":47352,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1338,"record_offset":49840,"parent_record_offset":49816,"ancestor_record_offset":47352,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":47620,"parent_record_offset":47588,"ancestor_record_offset":47352,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":52880,"parent_record_offset":52856,"ancestor_record_offset":47352,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":47588,"parent_record_offset":47556,"ancestor_record_offset":47352,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":48692,"parent_record_offset":48664,"ancestor_record_offset":47352,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":47556,"parent_record_offset":47524,"ancestor_record_offset":47352,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1338,"record_offset":48664,"parent_record_offset":48636,"ancestor_record_offset":47352,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1338,"record_offset":48636,"parent_record_offset":48608,"ancestor_record_offset":47352,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1338,"record_offset":48608,"parent_record_offset":48560,"ancestor_record_offset":47352,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":50336,"parent_record_offset":50112,"ancestor_record_offset":47352,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":52904,"parent_record_offset":52880,"ancestor_record_offset":47352,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":50308,"parent_record_offset":50112,"ancestor_record_offset":47352,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":50160,"parent_record_offset":50112,"ancestor_record_offset":47352,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":50476,"parent_record_offset":50112,"ancestor_record_offset":47352,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1338,"record_offset":47660,"parent_record_offset":47620,"ancestor_record_offset":47352,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":50020,"parent_record_offset":49980,"ancestor_record_offset":47352,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":50064,"parent_record_offset":50020,"ancestor_record_offset":47352,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1338,"record_offset":49980,"parent_record_offset":49940,"ancestor_record_offset":47352,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1338,"record_offset":50508,"parent_record_offset":50064,"ancestor_record_offset":47352,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1338,"record_offset":50112,"parent_record_offset":50064,"ancestor_record_offset":47352,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1338,"record_offset":47768,"parent_record_offset":47724,"ancestor_record_offset":47352,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1338,"record_offset":47792,"parent_record_offset":47768,"ancestor_record_offset":47352,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1338,"record_offset":48012,"parent_record_offset":47988,"ancestor_record_offset":47352,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1338,"record_offset":53044,"parent_record_offset":53016,"ancestor_record_offset":47352,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1338,"record_offset":47988,"parent_record_offset":47620,"ancestor_record_offset":47352,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":49768,"parent_record_offset":49196,"ancestor_record_offset":47352,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1338,"record_offset":50392,"parent_record_offset":50112,"ancestor_record_offset":47352,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":47900,"parent_record_offset":47872,"ancestor_record_offset":47352,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01338.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1338,"record_offset":50416,"parent_record_offset":50392,"ancestor_record_offset":47352,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/01338.jsonl.gz"}
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
// 完整结构及原始类型记录: debug_viz.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::debug_viz::DebugColor
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::debug_viz::DebugLine
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 8; 过程引用数: 6
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::debug_viz::DebugRectangle2d
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::debug_viz::DebugRectangle3d
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 4
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::debug_viz::DebugSphere
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 5
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::debug_viz::DebugTransform
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: debug_viz.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: debug_viz.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::event::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >,bevy_ecs::event::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::event::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >", "bevy_ecs::event::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>,bevy_ecs::event::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>,bevy_ecs::event::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>", "bevy_ecs::event::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>", "bevy_ecs::event::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::NextSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<country_core::resources::window::CursorMoved>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::event::EventWriter<country_core::resources::window::CursorMoved>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::window::CursorMoved>

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugColor::as_array
// 信息 ID: 2b052d070aa9159f017dc219
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::debug_viz::impl$3::show::closure$0; module=1338, inline_record=30484; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::debug_viz::impl$3::show::closure$1; module=1338, inline_record=34684; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::debug_viz::impl$4::show::closure$0; module=1338, inline_record=41660; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::debug_viz::DebugLine::show; module=1338, inline_record=44284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugLine::color
// 信息 ID: 0c57eb4c6007295fd8e2e0c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [静态直接引用] system_glade_border::display_glade_border::display_glade_border @ 0x1432b34 -> 0xec6110 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad9930 -> 0xec6110 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=416, inline_record=24656; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=416, inline_record=25432; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=416, inline_record=27472; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze; module=486, inline_record=35692; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::visualize_colliders::visualize_shape; module=1448, inline_record=10920; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::visualize_colliders::visualize_shape; module=1448, inline_record=11772; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugLine::dashed
// 信息 ID: f366a8ec7884b613ac888a17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0afcd -> 0xec6170 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::blueprint::show_max_wall_y @ 0x17a2d5f -> 0xec6170 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x1c3629b -> 0xec6170 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=391, inline_record=25336; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=391, inline_record=25364; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=391, inline_record=25784; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=1390, inline_record=30196; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=1390, inline_record=33480; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=1390, inline_record=33792; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=1390, inline_record=33820; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugLine::line_width
// 信息 ID: 8fc78f139a06ed48bc2457db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0afc2 -> 0xec61a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_glade_border::display_glade_border::display_glade_border @ 0x1432b50 -> 0xec61a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::blueprint::show_max_wall_y @ 0x17a2d78 -> 0xec61a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x1b5bc47 -> 0xec61a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x1b5bd27 -> 0xec61a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x1b5bdc2 -> 0xec61a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x1c362b6 -> 0xec61a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state; module=445, inline_record=44104; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display; module=461, inline_record=41660; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height; module=1302, inline_record=23728; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=28268; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugLine::new
// 信息 ID: 92ff97d5fdaef1d63324bf9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0afa8 -> 0xec60d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_glade_border::display_glade_border::display_glade_border @ 0x1432b15 -> 0xec60d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$0::call<tuple$<utils::geometry::ray::Ray,f32>,system_critters::wren_boids::sim_wren_boids::closure_env$3> @ 0x159fa07 -> 0xec60d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids::closure$3 @ 0x15ab3fc -> 0xec60d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::blueprint::show_max_wall_y @ 0x17a2d4e -> 0xec60d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad990f -> 0xec60d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_wrens @ 0x1b3d402 -> 0xec60d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x1b5bc33 -> 0xec60d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x1b5bd13 -> 0xec60d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x1b5bdae -> 0xec60d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x1c36288 -> 0xec60d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_rotate_lantern::ui_rotate_lantern @ 0x1c74db8 -> 0xec60d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=416, inline_record=25516; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::visualize_colliders::visualize_shape; module=1448, inline_record=10696; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::visualize_colliders::visualize_shape; module=1448, inline_record=10892; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugLine::set_ui_pass
// 信息 ID: 84adebb0e4a7d398bbb5e2e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_glade_border::display_glade_border::display_glade_border @ 0x1432b67 -> 0xec6140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugLine::show
// 信息 ID: cc17274e97504a25e23f3721
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 24 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0x585e64 -> 0xec61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0x585f83 -> 0xec61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0x586082 -> 0xec61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0x586467 -> 0xec61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state @ 0x5fc2f5 -> 0xec61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display @ 0x63536e -> 0xec61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze @ 0x69b8aa -> 0xec61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0afdc -> 0xec61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height @ 0xd1f560 -> 0xec61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::visualize_colliders::visualize_shape @ 0x110c826 -> 0xec61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::visualize_colliders::visualize_shape @ 0x110c950 -> 0xec61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::visualize_colliders::visualize_shape @ 0x110cb08 -> 0xec61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_glade_border::display_glade_border::display_glade_border @ 0x1432b7d -> 0xec61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$0::call<tuple$<utils::geometry::ray::Ray,f32>,system_critters::wren_boids::sim_wren_boids::closure_env$3> @ 0x159fa16 -> 0xec61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids::closure$3 @ 0x15ab40b -> 0xec61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::blueprint::show_max_wall_y @ 0x17a2d88 -> 0xec61d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle2d::color
// 信息 ID: ba0b9c7ce6b048b63f1b4e4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=416, inline_record=26816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle2d::new
// 信息 ID: 13b3e2a7a81a86587d9fd98e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=416, inline_record=24024; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=416, inline_record=26788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle2d::show
// 信息 ID: d9494c787eda69dc383861ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0x585d71 -> 0xec58e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0x586347 -> 0xec58e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle3d::color
// 信息 ID: de2d80ecdb9b2e2967c1a723
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::process_clutter_candidates::process_clutter_candidates @ 0x1acba83 -> 0xec4190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad9b94 -> 0xec4190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad94dc -> 0xec4190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad90fe -> 0xec4190 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle3d::line_width
// 信息 ID: 373890e6e9401428c0a32784
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad94f4 -> 0xec41c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad9116 -> 0xec41c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle3d::new
// 信息 ID: e310106a4c0ca67657598667
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::process_clutter_candidates::process_clutter_candidates @ 0x1acba6f -> 0xec4160 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad9b73 -> 0xec4160 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad94c1 -> 0xec4160 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad90e3 -> 0xec4160 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle3d::show
// 信息 ID: ff4ef172a213c4cc5e4ebdd7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::process_clutter_candidates::process_clutter_candidates @ 0x1acba90 -> 0xec41f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad9ba7 -> 0xec41f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad9504 -> 0xec41f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad9126 -> 0xec41f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugSphere::color
// 信息 ID: 5a2d5696673a296625c374fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::wren::update_wrens @ 0x1b3ddff -> 0xec38b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve; module=453, inline_record=33548; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugSphere::line_width
// 信息 ID: 4ad6be290429dee239af6773
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve; module=453, inline_record=33368; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve; module=453, inline_record=33576; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve; module=453, inline_record=33752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugSphere::new
// 信息 ID: 3727b80e120279615826eb97
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0a9e8 -> 0xec3880 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_wrens @ 0x1b3ddcc -> 0xec3880 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_rotate_lantern::ui_rotate_lantern @ 0x1c749b8 -> 0xec3880 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve; module=453, inline_record=33192; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugSphere::radius
// 信息 ID: be1f080384947a864e6c7989
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0aa02 -> 0xec38d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_wrens @ 0x1b3ddda -> 0xec38d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=391, inline_record=25308; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=391, inline_record=25756; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugSphere::show
// 信息 ID: 4c2ae86011410dbda2b1adda
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve @ 0x61e79d -> 0xec3910 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve @ 0x61e7f8 -> 0xec3910 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve @ 0x61e84f -> 0xec3910 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve @ 0x61e89f -> 0xec3910 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0aa1c -> 0xec3910 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_wrens @ 0x1b3de0f -> 0xec3910 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_rotate_lantern::ui_rotate_lantern @ 0x1c749d1 -> 0xec3910 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugSphere::solid
// 信息 ID: 9a7c1077f35c3274fc9b9156
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0aa0d -> 0xec38f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$1::show::closure$0
// 信息 ID: 7f838227afb7a303fec5b222
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::debug_viz::impl$1::show", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$1::show::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$1::show::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$1::show::closure_env$0 <- core::ptr::drop_in_place<country_core::utils::debug_viz::impl$1::show::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$1::show::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xec30e2 -> 0xec3bc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$3::show::closure$0
// 信息 ID: b22dc5be29bac24a36e1b779
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::debug_viz::impl$3::show", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$3::show::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$3::show::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$3::show::closure_env$0 <- core::ptr::drop_in_place<country_core::utils::debug_viz::impl$3::show::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$3::show::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xec31d3 -> 0xec4c20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$3::show::closure$1
// 信息 ID: 3ca4f49568d7cc69357f158a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::debug_viz::impl$3::show", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$3::show::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$3::show::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$3::show::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xec31a2 -> 0xec52a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$4::show::closure$0
// 信息 ID: 7a976ba7dae629e19d877fc5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::debug_viz::impl$4::show", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$4::show::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$4::show::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$4::show::closure_env$0 <- core::ptr::drop_in_place<country_core::utils::debug_viz::impl$4::show::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$4::show::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xec3113 -> 0xec5c90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$5::show::closure$0
// 信息 ID: e0833943792ca6feec708370
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::debug_viz::impl$5::show", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::debug_viz::DebugLine::show; module=1338, inline_record=44256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$5::show::closure$1
// 信息 ID: 87fbdf3524361072ee392965
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::debug_viz::impl$5::show", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$5::show::closure_env$1 <- core::ptr::drop_in_place<alloc::sync::ArcInner<country_core::utils::debug_viz::impl$5::show::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$5::show::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$5::show::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$5::show::closure_env$1 <- core::ptr::drop_in_place<country_core::utils::debug_viz::impl$5::show::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 3 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$5::show::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xec3044 -> 0xec6800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$6::fmt
// 信息 ID: 011b9b320ab55d6df27c1465
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$7::fmt
// 信息 ID: c0342be0062bec3456c04898
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
