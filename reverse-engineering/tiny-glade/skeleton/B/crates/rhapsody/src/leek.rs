// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/leek.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 1555b8ba1b465fcbbdd35137b830106eb1034dae7ed30bc3aa54bba5751d7b32
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: leek.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 691, 1978
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::leek::impl$0::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":691,"record_offset":964,"rva":12461840,"va":5381170960}
// 编译实例 RVAs: 0xbe2710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$core..fmt..Debug$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1727470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381170960,"raw_hex":"e8ee5b1a00","size":5,"soft_fail":false,"target_va":5382898528,"va":5381171053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: leek.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6089580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381170960,"raw_hex":"e89414a3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381171079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1666458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381170960,"raw_hex":"e89a6d1900","size":5,"soft_fail":false,"target_va":5382837616,"va":5381171153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30310673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381170960,"raw_hex":"e81181ce01","size":5,"soft_fail":false,"target_va":5411481920,"va":5381171242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":691,"record_offset":1096,"parent_record_offset":964,"ancestor_record_offset":964,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":691,"record_offset":1648,"parent_record_offset":964,"ancestor_record_offset":964,"inlinee":6414,"source_line":1419,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$24::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":691,"record_offset":1408,"parent_record_offset":964,"ancestor_record_offset":964,"inlinee":6294,"source_line":2586,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":691,"record_offset":1124,"parent_record_offset":964,"ancestor_record_offset":964,"inlinee":4555,"source_line":56,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":691,"record_offset":1152,"parent_record_offset":964,"ancestor_record_offset":964,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":691,"record_offset":1436,"parent_record_offset":964,"ancestor_record_offset":964,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":691,"record_offset":1596,"parent_record_offset":1572,"ancestor_record_offset":964,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":691,"record_offset":1572,"parent_record_offset":1520,"ancestor_record_offset":964,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":691,"record_offset":1736,"parent_record_offset":1712,"ancestor_record_offset":964,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":691,"record_offset":2008,"parent_record_offset":1980,"ancestor_record_offset":964,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":691,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":964,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":691,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":964,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":691,"record_offset":1520,"parent_record_offset":1496,"ancestor_record_offset":964,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":691,"record_offset":2068,"parent_record_offset":2036,"ancestor_record_offset":964,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":691,"record_offset":1784,"parent_record_offset":1760,"ancestor_record_offset":964,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":691,"record_offset":1760,"parent_record_offset":1736,"ancestor_record_offset":964,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":691,"record_offset":2036,"parent_record_offset":2008,"ancestor_record_offset":964,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":691,"record_offset":1496,"parent_record_offset":1476,"ancestor_record_offset":964,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":691,"record_offset":1944,"parent_record_offset":1916,"ancestor_record_offset":964,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":691,"record_offset":1712,"parent_record_offset":1676,"ancestor_record_offset":964,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":691,"record_offset":1888,"parent_record_offset":1860,"ancestor_record_offset":964,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":691,"record_offset":2264,"parent_record_offset":1944,"ancestor_record_offset":964,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":691,"record_offset":1676,"parent_record_offset":1648,"ancestor_record_offset":964,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":691,"record_offset":1980,"parent_record_offset":1944,"ancestor_record_offset":964,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":691,"record_offset":1916,"parent_record_offset":1888,"ancestor_record_offset":964,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":691,"record_offset":1232,"parent_record_offset":1184,"ancestor_record_offset":964,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":691,"record_offset":1256,"parent_record_offset":1232,"ancestor_record_offset":964,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":691,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":964,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":691,"record_offset":1344,"parent_record_offset":1184,"ancestor_record_offset":964,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":691,"record_offset":1184,"parent_record_offset":1152,"ancestor_record_offset":964,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":691,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":964,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":691,"record_offset":1456,"parent_record_offset":1436,"ancestor_record_offset":964,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":691,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":964,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":691,"record_offset":1824,"parent_record_offset":1676,"ancestor_record_offset":964,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::leek::impl$1::from
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=16
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9998
// 首条位置: {"module":691,"record_offset":2540,"rva":12462208,"va":5381171328}
// 编译实例 RVAs: 0xbe2880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30370698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381171328,"raw_hex":"e88a6bcf01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381171593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":691,"record_offset":2908,"parent_record_offset":2540,"ancestor_record_offset":2540,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_direct] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":691,"record_offset":2656,"parent_record_offset":2540,"ancestor_record_offset":2540,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":691,"record_offset":2756,"parent_record_offset":2720,"ancestor_record_offset":2540,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":691,"record_offset":2720,"parent_record_offset":2676,"ancestor_record_offset":2540,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":691,"record_offset":2676,"parent_record_offset":2656,"ancestor_record_offset":2540,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=45
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":691,"record_offset":2816,"parent_record_offset":2676,"ancestor_record_offset":2540,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":691,"record_offset":2788,"parent_record_offset":2676,"ancestor_record_offset":2540,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00691.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::leek::impl$2::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":691,"record_offset":2948,"rva":12462480,"va":5381171600}
// 编译实例 RVAs: 0xbe2990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1716378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381171600,"raw_hex":"e89a301a00","size":5,"soft_fail":false,"target_va":5382888144,"va":5381171761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":691,"record_offset":3060,"parent_record_offset":2948,"ancestor_record_offset":2948,"inlinee":7062,"source_line":1917,"archive":"_pdb/B/modules/00691.jsonl.gz"}
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
// 完整结构及原始类型记录: leek.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: leek.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: leek.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::leek::impl$0::fmt
// 信息 ID: a136fe6b25aaab37d96ad091
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::leek::impl$1::from
// 信息 ID: b6a6014e57cd810aed55d17a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 14 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > > @ 0xf86a8 -> 0xbe2880 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$1::record_batch::closure$1<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c3405 -> 0xbe2880 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain> >; module=1978, inline_record=5140; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain> >; module=1978, inline_record=5564; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler> >; module=1978, inline_record=8604; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler> >; module=1978, inline_record=9028; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass> >; module=1978, inline_record=12076; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass> >; module=1978, inline_record=12500; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture> >; module=1978, inline_record=15540; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture> >; module=1978, inline_record=15964; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::leek::impl$2::fmt
// 信息 ID: 87e8f58309f35cede5bd1313
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
