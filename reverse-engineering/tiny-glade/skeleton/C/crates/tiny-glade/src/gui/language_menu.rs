// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/gui/language_menu.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: c5e477777f0dee5c5db71e3ebf747c74a8a5fd4df045ad9ef03ba6db81f26174
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: language_menu.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 35, 67
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
// 首条位置: {"module":35,"record_offset":10660,"rva":1600064,"va":5370309184}
// 编译实例 RVAs: 0x186a40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=4
//     target_aliases=17 candidates; complete list: language_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-768096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370309184,"raw_hex":"e8a047f4ff","size":5,"soft_fail":false,"target_va":5369542000,"va":5370310091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140222d00
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: language_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t638575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370309184,"raw_hex":"e86fbe0900","size":5,"soft_fail":false,"target_va":5370948864,"va":5370310284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: language_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3174261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370309184,"raw_hex":"e8756f3000","size":5,"soft_fail":false,"target_va":5373484384,"va":5370310118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: language_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3174641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370309184,"raw_hex":"e8f1703000","size":5,"soft_fail":false,"target_va":5373484784,"va":5370310138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5898203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370309184,"raw_hex":"e8dbff5900","size":5,"soft_fail":false,"target_va":5376208368,"va":5370310160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5899297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370309184,"raw_hex":"e821045a00","size":5,"soft_fail":false,"target_va":5376209536,"va":5370310234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::gap
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5943562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370309184,"raw_hex":"e80ab15a00","size":5,"soft_fail":false,"target_va":5376253776,"va":5370310209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5943342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370309184,"raw_hex":"e82eb05a00","size":5,"soft_fail":false,"target_va":5376253520,"va":5370310173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":35,"record_offset":10796,"parent_record_offset":10660,"ancestor_record_offset":10660,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":35,"record_offset":10844,"parent_record_offset":10820,"ancestor_record_offset":10660,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":35,"record_offset":10876,"parent_record_offset":10820,"ancestor_record_offset":10660,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":35,"record_offset":10820,"parent_record_offset":10796,"ancestor_record_offset":10660,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::language_menu::do_gui_language_menu::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27FB
// 首条位置: {"module":67,"inlinee":10235}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":67,"record_offset":14032,"parent_record_offset":13816,"ancestor_record_offset":13816,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":67,"record_offset":14688,"parent_record_offset":13816,"ancestor_record_offset":13816,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":67,"record_offset":13960,"parent_record_offset":13816,"ancestor_record_offset":13816,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":67,"record_offset":13896,"parent_record_offset":13816,"ancestor_record_offset":13816,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":67,"record_offset":14504,"parent_record_offset":14480,"ancestor_record_offset":13816,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":67,"record_offset":14528,"parent_record_offset":14504,"ancestor_record_offset":13816,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":67,"record_offset":14888,"parent_record_offset":14864,"ancestor_record_offset":13816,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":67,"record_offset":14912,"parent_record_offset":14888,"ancestor_record_offset":13816,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":67,"record_offset":14552,"parent_record_offset":14528,"ancestor_record_offset":13816,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":67,"record_offset":14936,"parent_record_offset":14912,"ancestor_record_offset":13816,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":67,"record_offset":14480,"parent_record_offset":14332,"ancestor_record_offset":13816,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":67,"record_offset":14864,"parent_record_offset":14808,"ancestor_record_offset":13816,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":67,"record_offset":14264,"parent_record_offset":14232,"ancestor_record_offset":13816,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":67,"record_offset":14836,"parent_record_offset":14808,"ancestor_record_offset":13816,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":67,"record_offset":14808,"parent_record_offset":14784,"ancestor_record_offset":13816,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":67,"record_offset":14332,"parent_record_offset":14296,"ancestor_record_offset":13816,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":67,"record_offset":14296,"parent_record_offset":14264,"ancestor_record_offset":13816,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":67,"record_offset":14784,"parent_record_offset":14760,"ancestor_record_offset":13816,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":67,"record_offset":14368,"parent_record_offset":14332,"ancestor_record_offset":13816,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":67,"record_offset":14128,"parent_record_offset":14096,"ancestor_record_offset":13816,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":67,"record_offset":14160,"parent_record_offset":14128,"ancestor_record_offset":13816,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":67,"record_offset":14192,"parent_record_offset":14160,"ancestor_record_offset":13816,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":67,"record_offset":14096,"parent_record_offset":14064,"ancestor_record_offset":13816,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":67,"record_offset":14064,"parent_record_offset":14032,"ancestor_record_offset":13816,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":67,"record_offset":14232,"parent_record_offset":14192,"ancestor_record_offset":13816,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":67,"record_offset":14392,"parent_record_offset":14368,"ancestor_record_offset":13816,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":67,"record_offset":14440,"parent_record_offset":14416,"ancestor_record_offset":13816,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":67,"record_offset":14416,"parent_record_offset":14392,"ancestor_record_offset":13816,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":67,"record_offset":14608,"parent_record_offset":14192,"ancestor_record_offset":13816,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":67,"record_offset":14632,"parent_record_offset":14608,"ancestor_record_offset":13816,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":67,"record_offset":14000,"parent_record_offset":13960,"ancestor_record_offset":13816,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":67,"record_offset":13924,"parent_record_offset":13896,"ancestor_record_offset":13816,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00067.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::gui::language_menu::do_gui_language_menu
// 信息 ID: ddafe9490d17addae069c8bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::language_menu::do_gui_language_menu::closure$0
// 信息 ID: a433c7873faaaa1e5393f0b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::language_menu::do_gui_language_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0 <- toolbar::mei::core::NodeContext::with<tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0>; module=67, inline_record=13816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
