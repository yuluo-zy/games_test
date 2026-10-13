// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/game_key_bindings.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: bee5c235f6a2e6bcd618e3aad0b13d0e4d306a5690f01e709a36b5f94e69d29f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: game_key_bindings.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 393, 407, 426, 1373
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_input; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use ron; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBinding::action_desc
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":393,"record_offset":21188,"rva":5375392,"va":5374084512}
// 编译实例 RVAs: 0x5205a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1336281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374084512,"raw_hex":"e8279cebff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374085972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29507353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374084512,"raw_hex":"e8193fc201","size":5,"soft_fail":false,"target_va":5403593360,"va":5374086002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28892084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374084512,"raw_hex":"e8b4dbb801","size":5,"soft_fail":false,"target_va":5402978144,"va":5374086055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t3445116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374084512,"raw_hex":"e87c913400","size":5,"soft_fail":false,"target_va":5377529888,"va":5374084767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3441073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374084512,"raw_hex":"e8b1813400","size":5,"soft_fail":false,"target_va":5377527120,"va":5374086042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t2068204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374084512,"raw_hex":"e8ec8e1f00","size":5,"soft_fail":false,"target_va":5376152880,"va":5374084671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":393,"record_offset":22052,"parent_record_offset":21188,"ancestor_record_offset":21188,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_key_bindings::KeyBindingDesc::icon
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":393,"record_offset":21336,"parent_record_offset":21188,"ancestor_record_offset":21188,"inlinee":22486,"source_line":301,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_key_bindings::KeyBindingDesc::text
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":393,"record_offset":22136,"parent_record_offset":21188,"ancestor_record_offset":21188,"inlinee":22485,"source_line":294,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":393,"record_offset":21828,"parent_record_offset":21804,"ancestor_record_offset":21188,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":393,"record_offset":21852,"parent_record_offset":21828,"ancestor_record_offset":21188,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":393,"record_offset":21804,"parent_record_offset":21760,"ancestor_record_offset":21188,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":393,"record_offset":22100,"parent_record_offset":22076,"ancestor_record_offset":21188,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":393,"record_offset":21760,"parent_record_offset":21732,"ancestor_record_offset":21188,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":393,"record_offset":21732,"parent_record_offset":21704,"ancestor_record_offset":21188,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":21668,"parent_record_offset":21640,"ancestor_record_offset":21188,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":21640,"parent_record_offset":21612,"ancestor_record_offset":21188,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":21584,"parent_record_offset":21556,"ancestor_record_offset":21188,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":21612,"parent_record_offset":21584,"ancestor_record_offset":21188,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":21556,"parent_record_offset":21528,"ancestor_record_offset":21188,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":393,"record_offset":21528,"parent_record_offset":21500,"ancestor_record_offset":21188,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":393,"record_offset":21500,"parent_record_offset":21472,"ancestor_record_offset":21188,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":393,"record_offset":21704,"parent_record_offset":21668,"ancestor_record_offset":21188,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":393,"record_offset":21472,"parent_record_offset":21392,"ancestor_record_offset":21188,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":393,"record_offset":21956,"parent_record_offset":21932,"ancestor_record_offset":21188,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":393,"record_offset":22076,"parent_record_offset":22052,"ancestor_record_offset":21188,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":393,"record_offset":21932,"parent_record_offset":21668,"ancestor_record_offset":21188,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":393,"record_offset":21880,"parent_record_offset":21852,"ancestor_record_offset":21188,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::MouseBinding::action_desc
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":393,"record_offset":17420,"rva":5374704,"va":5374083824}
// 编译实例 RVAs: 0x5202f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1334230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083824,"raw_hex":"e82aa4ebff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374083921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29509404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083824,"raw_hex":"e81c47c201","size":5,"soft_fail":false,"target_va":5403593360,"va":5374083951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28894138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083824,"raw_hex":"e8bae3b801","size":5,"soft_fail":false,"target_va":5402978144,"va":5374084001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3443127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083824,"raw_hex":"e8b7893400","size":5,"soft_fail":false,"target_va":5377527120,"va":5374083988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2068987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083824,"raw_hex":"e8fb911f00","size":5,"soft_fail":false,"target_va":5376152880,"va":5374083888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::game_key_bindings::KeyBindingDesc::text
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":393,"record_offset":17564,"parent_record_offset":17420,"ancestor_record_offset":17420,"inlinee":22485,"source_line":294,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":393,"record_offset":17940,"parent_record_offset":17916,"ancestor_record_offset":17420,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":393,"record_offset":17964,"parent_record_offset":17940,"ancestor_record_offset":17420,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":393,"record_offset":17916,"parent_record_offset":17880,"ancestor_record_offset":17420,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":393,"record_offset":17880,"parent_record_offset":17852,"ancestor_record_offset":17420,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":393,"record_offset":17852,"parent_record_offset":17824,"ancestor_record_offset":17420,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":393,"record_offset":18032,"parent_record_offset":17852,"ancestor_record_offset":17420,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":17792,"parent_record_offset":17764,"ancestor_record_offset":17420,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":17764,"parent_record_offset":17736,"ancestor_record_offset":17420,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":17708,"parent_record_offset":17680,"ancestor_record_offset":17420,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":17736,"parent_record_offset":17708,"ancestor_record_offset":17420,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":17680,"parent_record_offset":17652,"ancestor_record_offset":17420,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":393,"record_offset":17652,"parent_record_offset":17624,"ancestor_record_offset":17420,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":393,"record_offset":17624,"parent_record_offset":17596,"ancestor_record_offset":17420,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":393,"record_offset":17824,"parent_record_offset":17792,"ancestor_record_offset":17420,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":393,"record_offset":17596,"parent_record_offset":17564,"ancestor_record_offset":17420,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":393,"record_offset":18096,"parent_record_offset":18072,"ancestor_record_offset":17420,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":393,"record_offset":18072,"parent_record_offset":17792,"ancestor_record_offset":17420,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":393,"record_offset":17988,"parent_record_offset":17964,"ancestor_record_offset":17420,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::mouse_button_to_string
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":393,"record_offset":18168,"rva":5374896,"va":5374084016}
// 编译实例 RVAs: 0x5203b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1334372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374084016,"raw_hex":"e89ca3ebff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374084063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28893673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374084016,"raw_hex":"e8e9e1b801","size":5,"soft_fail":false,"target_va":5402978144,"va":5374084466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3445461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374084016,"raw_hex":"e8d5923400","size":5,"soft_fail":false,"target_va":5377529888,"va":5374084422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":393,"record_offset":21072,"parent_record_offset":18168,"ancestor_record_offset":18168,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$44::from
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":393,"record_offset":18312,"parent_record_offset":18168,"ancestor_record_offset":18168,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":393,"record_offset":18584,"parent_record_offset":18560,"ancestor_record_offset":18168,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":393,"record_offset":18608,"parent_record_offset":18584,"ancestor_record_offset":18168,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":393,"record_offset":18560,"parent_record_offset":18532,"ancestor_record_offset":18168,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":393,"record_offset":21120,"parent_record_offset":21096,"ancestor_record_offset":18168,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":393,"record_offset":18532,"parent_record_offset":18508,"ancestor_record_offset":18168,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":393,"record_offset":18508,"parent_record_offset":18484,"ancestor_record_offset":18168,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":393,"record_offset":19188,"parent_record_offset":19016,"ancestor_record_offset":18168,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":18456,"parent_record_offset":18432,"ancestor_record_offset":18168,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":18432,"parent_record_offset":18408,"ancestor_record_offset":18168,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":18384,"parent_record_offset":18360,"ancestor_record_offset":18168,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":18408,"parent_record_offset":18384,"ancestor_record_offset":18168,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":18360,"parent_record_offset":18336,"ancestor_record_offset":18168,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":393,"record_offset":18336,"parent_record_offset":18312,"ancestor_record_offset":18168,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":393,"record_offset":20828,"parent_record_offset":20800,"ancestor_record_offset":18168,"inlinee":7911,"source_line":993,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":393,"record_offset":18484,"parent_record_offset":18456,"ancestor_record_offset":18168,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":393,"record_offset":18708,"parent_record_offset":18684,"ancestor_record_offset":18168,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":393,"record_offset":21096,"parent_record_offset":21072,"ancestor_record_offset":18168,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":393,"record_offset":18684,"parent_record_offset":18456,"ancestor_record_offset":18168,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":393,"record_offset":18632,"parent_record_offset":18608,"ancestor_record_offset":18168,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5B91
// 首条位置: {"module":1373,"inlinee":23441}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":1373,"record_offset":64412,"parent_record_offset":64388,"ancestor_record_offset":64388,"inlinee":8724,"source_line":221,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":407,"record_offset":40368,"rva":5659232,"va":5374368352}
// 编译实例 RVAs: 0x565a60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29224910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374368352,"raw_hex":"e8ceefbd01","size":5,"soft_fail":false,"target_va":5403593344,"va":5374368429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":407,"record_offset":40556,"parent_record_offset":40368,"ancestor_record_offset":40368,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00407.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":407,"record_offset":40584,"parent_record_offset":40556,"ancestor_record_offset":40368,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00407.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":407,"record_offset":40612,"parent_record_offset":40584,"ancestor_record_offset":40368,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00407.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":407,"record_offset":40640,"parent_record_offset":40612,"ancestor_record_offset":40368,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00407.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF805
// 首条位置: {"module":1373,"inlinee":63493}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":1373,"record_offset":54316,"parent_record_offset":54288,"ancestor_record_offset":54288,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1373,"record_offset":54352,"parent_record_offset":54316,"ancestor_record_offset":54288,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1373,"record_offset":54416,"parent_record_offset":54316,"ancestor_record_offset":54288,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1373,"record_offset":54388,"parent_record_offset":54316,"ancestor_record_offset":54288,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF803
// 首条位置: {"module":1373,"inlinee":63491}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1373,"record_offset":54632,"parent_record_offset":53532,"ancestor_record_offset":53532,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1373,"record_offset":54560,"parent_record_offset":53532,"ancestor_record_offset":53532,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1373,"record_offset":53572,"parent_record_offset":53532,"ancestor_record_offset":53532,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1373,"record_offset":54532,"parent_record_offset":53532,"ancestor_record_offset":53532,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1373,"record_offset":53784,"parent_record_offset":53608,"ancestor_record_offset":53532,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1373,"record_offset":53824,"parent_record_offset":53784,"ancestor_record_offset":53532,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1373,"record_offset":53960,"parent_record_offset":53608,"ancestor_record_offset":53532,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1373,"record_offset":54456,"parent_record_offset":53608,"ancestor_record_offset":53532,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1373,"record_offset":53740,"parent_record_offset":53608,"ancestor_record_offset":53532,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1373,"record_offset":54416,"parent_record_offset":54316,"ancestor_record_offset":53532,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":1373,"record_offset":53928,"parent_record_offset":53904,"ancestor_record_offset":53532,"inlinee":63493,"source_line":4,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/id.rs
//     evidence={"module":1373,"record_offset":53864,"parent_record_offset":53608,"ancestor_record_offset":53532,"inlinee":10226,"source_line":11,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":1373,"record_offset":54316,"parent_record_offset":54288,"ancestor_record_offset":53532,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":1373,"record_offset":53608,"parent_record_offset":53572,"ancestor_record_offset":53532,"inlinee":10224,"source_line":838,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1373,"record_offset":54388,"parent_record_offset":54316,"ancestor_record_offset":53532,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1373,"record_offset":53904,"parent_record_offset":53608,"ancestor_record_offset":53532,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":407,"record_offset":38756,"rva":5654240,"va":5374363360}
// 编译实例 RVAs: 0x5646e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400105d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5589407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374363360,"raw_hex":"e861b6aaff","size":5,"soft_fail":false,"target_va":5368776144,"va":5374365546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f41740
//     owner=unresolved_namespace; count=28
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10342320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374363360,"raw_hex":"e8b0cf9d00","size":5,"soft_fail":false,"target_va":5384705856,"va":5374363531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f41c90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10341862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374363360,"raw_hex":"e8e6cd9d00","size":5,"soft_fail":false,"target_va":5384707216,"va":5374365349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f42cc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10347925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374363360,"raw_hex":"e895e59d00","size":5,"soft_fail":false,"target_va":5384711360,"va":5374363430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":407,"record_offset":40180,"parent_record_offset":38756,"ancestor_record_offset":38756,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00407.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":407,"record_offset":38972,"parent_record_offset":38756,"ancestor_record_offset":38756,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00407.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":407,"record_offset":40124,"parent_record_offset":38756,"ancestor_record_offset":38756,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00407.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":407,"record_offset":40232,"parent_record_offset":40204,"ancestor_record_offset":38756,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/00407.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$5::drop
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/ser/mod.rs
//     evidence={"module":407,"record_offset":40204,"parent_record_offset":40180,"ancestor_record_offset":38756,"inlinee":4979,"source_line":1098,"archive":"_pdb/A/modules/00407.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$0::default
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x57E3
// 首条位置: {"module":393,"record_offset":11440,"rva":5370832,"va":5374079952}
// 编译实例 RVAs: 0x51f3d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBindings::assign_mouse_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":393,"record_offset":11564,"rva":5371120,"va":5374080240}
// 编译实例 RVAs: 0x51f4f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBindings::assign_key
// 记录形式: compiled_procedure; instances=1
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":393,"record_offset":11708,"rva":5371152,"va":5374080272}
// 编译实例 RVAs: 0x51f510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::game_key_bindings::impl$1::assign_key::closure$0
//     owner=private_source_ownership_candidate; count=27
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":393,"record_offset":11848,"parent_record_offset":11708,"ancestor_record_offset":11708,"inlinee":22483,"source_line":95,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$20::eq
//     owner=git_dependency_definition; count=27
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/keyboard.rs
//     evidence={"module":393,"record_offset":11976,"parent_record_offset":11948,"ancestor_record_offset":11708,"inlinee":12074,"source_line":163,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$28::eq
//     owner=git_dependency_definition; count=27
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/keyboard.rs
//     evidence={"module":393,"record_offset":11948,"parent_record_offset":11916,"ancestor_record_offset":11708,"inlinee":12072,"source_line":200,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":393,"record_offset":11888,"parent_record_offset":11848,"ancestor_record_offset":11708,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":393,"record_offset":11916,"parent_record_offset":11888,"ancestor_record_offset":11708,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$1::assign_key::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=28
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x57D3
// 首条位置: {"module":393,"inlinee":22483}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":393,"record_offset":11888,"parent_record_offset":11848,"ancestor_record_offset":11848,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$20::eq
//     owner=git_dependency_definition; count=27
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/keyboard.rs
//     evidence={"module":393,"record_offset":11976,"parent_record_offset":11948,"ancestor_record_offset":11848,"inlinee":12074,"source_line":163,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$28::eq
//     owner=git_dependency_definition; count=27
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/keyboard.rs
//     evidence={"module":393,"record_offset":11948,"parent_record_offset":11916,"ancestor_record_offset":11848,"inlinee":12072,"source_line":200,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":393,"record_offset":11916,"parent_record_offset":11888,"ancestor_record_offset":11848,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::MouseBinding::get_mouse_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 228
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":393,"record_offset":17280,"rva":5374672,"va":5374083792}
// 编译实例 RVAs: 0x5202d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBindingDesc::text
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 294
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x57D5
// 首条位置: {"module":393,"inlinee":22485}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":393,"record_offset":17596,"parent_record_offset":17564,"ancestor_record_offset":17564,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":393,"record_offset":17940,"parent_record_offset":17916,"ancestor_record_offset":17564,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":393,"record_offset":17964,"parent_record_offset":17940,"ancestor_record_offset":17564,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":393,"record_offset":17916,"parent_record_offset":17880,"ancestor_record_offset":17564,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":393,"record_offset":17880,"parent_record_offset":17852,"ancestor_record_offset":17564,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":393,"record_offset":17852,"parent_record_offset":17824,"ancestor_record_offset":17564,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":393,"record_offset":18032,"parent_record_offset":17852,"ancestor_record_offset":17564,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":17792,"parent_record_offset":17764,"ancestor_record_offset":17564,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":17764,"parent_record_offset":17736,"ancestor_record_offset":17564,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":17708,"parent_record_offset":17680,"ancestor_record_offset":17564,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":17736,"parent_record_offset":17708,"ancestor_record_offset":17564,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":17680,"parent_record_offset":17652,"ancestor_record_offset":17564,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":393,"record_offset":17652,"parent_record_offset":17624,"ancestor_record_offset":17564,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":393,"record_offset":17624,"parent_record_offset":17596,"ancestor_record_offset":17564,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":393,"record_offset":17824,"parent_record_offset":17792,"ancestor_record_offset":17564,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":393,"record_offset":18096,"parent_record_offset":18072,"ancestor_record_offset":17564,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":393,"record_offset":18072,"parent_record_offset":17792,"ancestor_record_offset":17564,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":393,"record_offset":17988,"parent_record_offset":17964,"ancestor_record_offset":17564,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBindingDesc::icon
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x57D6
// 首条位置: {"module":393,"inlinee":22486}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":393,"record_offset":21472,"parent_record_offset":21392,"ancestor_record_offset":21392,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":393,"record_offset":21828,"parent_record_offset":21804,"ancestor_record_offset":21392,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":393,"record_offset":21852,"parent_record_offset":21828,"ancestor_record_offset":21392,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":393,"record_offset":21804,"parent_record_offset":21760,"ancestor_record_offset":21392,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":393,"record_offset":21760,"parent_record_offset":21732,"ancestor_record_offset":21392,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":393,"record_offset":21732,"parent_record_offset":21704,"ancestor_record_offset":21392,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":21668,"parent_record_offset":21640,"ancestor_record_offset":21392,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":21640,"parent_record_offset":21612,"ancestor_record_offset":21392,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":21584,"parent_record_offset":21556,"ancestor_record_offset":21392,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":21612,"parent_record_offset":21584,"ancestor_record_offset":21392,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":393,"record_offset":21556,"parent_record_offset":21528,"ancestor_record_offset":21392,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":393,"record_offset":21528,"parent_record_offset":21500,"ancestor_record_offset":21392,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":393,"record_offset":21500,"parent_record_offset":21472,"ancestor_record_offset":21392,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":393,"record_offset":21704,"parent_record_offset":21668,"ancestor_record_offset":21392,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":393,"record_offset":21956,"parent_record_offset":21932,"ancestor_record_offset":21392,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":393,"record_offset":21932,"parent_record_offset":21668,"ancestor_record_offset":21392,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":393,"record_offset":21880,"parent_record_offset":21852,"ancestor_record_offset":21392,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00393.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBinding::get_keycode
// 记录形式: compiled_procedure; instances=1
// 源码行: 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":393,"record_offset":22980,"rva":5376944,"va":5374086064}
// 编译实例 RVAs: 0x520bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::keycode_to_str
// 记录形式: compiled_procedure; instances=1
// 源码行: 381
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":393,"record_offset":23140,"rva":5377232,"va":5374086352}
// 编译实例 RVAs: 0x520cd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: game_key_bindings.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::game_key_bindings::KeyBinding
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::game_key_bindings::KeyBinding", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >", "constructor": "tuple$", "arguments": ["ref$<str$>", "alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>", "enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::game_key_bindings::KeyBindingDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::game_key_bindings::KeyBindingDesc>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::game_key_bindings::KeyBindingDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::game_key_bindings::KeyBindings
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::game_key_bindings::KeyBindings>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "country_core::game_key_bindings::KeyBindings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::game_key_bindings::MouseBinding
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::game_key_bindings::MouseBinding>", "constructor": "core::option::Option", "arguments": ["country_core::game_key_bindings::MouseBinding"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::game_key_bindings::MouseBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > >", "constructor": "tuple$", "arguments": ["ref$<str$>", "enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > >", "constructor": "core::option::Option", "arguments": ["tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > >", "constructor": "enum2$", "arguments": ["core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >", "constructor": "tuple$", "arguments": ["ref$<str$>", "alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>", "enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: game_key_bindings.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: game_key_bindings.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBinding::action_desc
// 信息 ID: 90a50b93e4b53791fd0ddb3d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$17 @ 0x3436f4 -> 0x5205a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBinding::get_keycode
// 信息 ID: bc2c45b426d317be6326f7ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls::closure$1 @ 0x33e37d -> 0x520bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$17 @ 0x34378c -> 0x520bb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBindingDesc::icon
// 信息 ID: 2f2036d4f205f3ce9ea3cc70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=393, inline_record=21336; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=393, inline_record=21364; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=393, inline_record=21392; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=393, inline_record=22024; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=393, inline_record=22164; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=393, inline_record=22304; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=393, inline_record=22556; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=393, inline_record=22584; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBindingDesc::text
// 信息 ID: b14c23011b5ca63be199a4d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_key_bindings::MouseBinding::action_desc; module=393, inline_record=17564; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=393, inline_record=22136; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=393, inline_record=22276; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=393, inline_record=22416; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=393, inline_record=22528; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=393, inline_record=22696; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=393, inline_record=22836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBindings::assign_key
// 信息 ID: 5896524a0d3b03cc041090af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$18 @ 0x343c38 -> 0x51f510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBindings::assign_mouse_button
// 信息 ID: 067b2944a773231f4fb09f77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$22 @ 0x34429e -> 0x51f4f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::MouseBinding::action_desc
// 信息 ID: dc843e2d04e83cea650fd68e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$21 @ 0x343df2 -> 0x5202f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::MouseBinding::get_mouse_button
// 信息 ID: ca822d6607a53dee0de78597
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls::closure$1 @ 0x33e310 -> 0x5202d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$21 @ 0x343e83 -> 0x5202d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize
// 信息 ID: 3829fe02ad8f6d6113697c24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_profile::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=64388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 56978acf196fd83dd5a7b6fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor> @ 0xf4f87a -> 0x565a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<country_core::game_key_bindings::_::impl$0::deserialize::__FieldVisitor> @ 0x1007633 -> 0x565a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<country_core::game_key_bindings::_::impl$0::deserialize::__FieldVisitor> @ 0x1007678 -> 0x565a60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 42e41ea8d0d0d280b4fd8de4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=53928; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=54136; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=54288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: e461a18af5dd0605a0c7abfc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=53532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 信息 ID: b356f2c8c9c5c6dc42307165
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::Serializer -> ["ref_mut$<alloc::string::String>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ron::ser::Serializer<ref_mut$<alloc::string::String> >"]; 字段布局 = unknown。
// [类型关系] country_core::game_key_bindings::_::impl$0::serialize -> ["ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::game_key_bindings::KeyBindings> @ 0xf40eb4 -> 0x5646e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$0::default
// 信息 ID: 3c330bdcbce1a339bbb05698
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$20 @ 0x343d30 -> 0x51f3d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::game_profile::load_game_profile; module=426, inline_record=10304; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$1::assign_key::closure$0
// 信息 ID: 527adf094f85b221ea122e1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::game_key_bindings::impl$1::assign_key", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 27 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=393, inline_record=11848; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=393, inline_record=12020; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=393, inline_record=12192; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=393, inline_record=12364; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=393, inline_record=12536; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=393, inline_record=12708; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=393, inline_record=12880; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=393, inline_record=13052; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::keycode_to_str
// 信息 ID: 398ad219c973a4723db50403
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls::closure$1 @ 0x33e3a8 -> 0x520cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$17 @ 0x343876 -> 0x520cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$17 @ 0x3437ba -> 0x520cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::ui::closure$6 @ 0x1620f89 -> 0x520cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_display_system::hint_draw::closure$1 @ 0x1aa6c45 -> 0x520cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_display_system::hint_draw::closure$1 @ 0x1aa680f -> 0x520cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_display_system::hint_draw::closure$1 @ 0x1aa6848 -> 0x520cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_display_system::hint_draw::closure$1 @ 0x1aa6985 -> 0x520cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_display_system::hint_draw::closure$1 @ 0x1aa69c1 -> 0x520cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_display_system::hint_draw::closure$1 @ 0x1aa6a0f -> 0x520cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_display_system::hint_draw::closure$1 @ 0x1aa6a47 -> 0x520cd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::mouse_button_to_string
// 信息 ID: 40c3574bbf8f466308b73b83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$21 @ 0x343e94 -> 0x5203b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$21 @ 0x343f31 -> 0x5203b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_display_system::hint_draw::closure$1::closure$2 @ 0x1aa6f49 -> 0x5203b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
