// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tg-package-integrity/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: df95b1f5a69a98c50f94caea14cb881578e2dbed48eec47ac27fecc3282d5f71
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 315, 1002, 1003, 2362
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use easy_parallel; // namespace candidate
//   package identities: [{"name":"easy-parallel","source":"PDB_registry_path_only","version":"3.2.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use tg_package_integrity; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::verify_package_manifest
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":315,"record_offset":1672,"rva":4430576,"va":5373139696}
// 编译实例 RVAs: 0x439af0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2873436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373139696,"raw_hex":"e85cd82b00","size":5,"soft_fail":false,"target_va":5376013504,"va":5373140063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4272651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373139696,"raw_hex":"e80b324100","size":5,"soft_fail":false,"target_va":5377412544,"va":5373139888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=3
//     target_aliases=89 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-825710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373139696,"raw_hex":"e89266f3ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5373139913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-821080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373139696,"raw_hex":"e8a878f3ff","size":5,"soft_fail":false,"target_va":5372320512,"va":5373141587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140382c20
//     owner=unresolved_namespace; count=2
//     target_aliases=37 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-751315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373139696,"raw_hex":"e82d89f4ff","size":5,"soft_fail":false,"target_va":5372390432,"va":5373141742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=17
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-390448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373139696,"raw_hex":"e8d00afaff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373140155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a42640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6326858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373139696,"raw_hex":"e84a8a6000","size":5,"soft_fail":false,"target_va":5379466816,"va":5373139953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a444c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6334368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373139696,"raw_hex":"e8a0a76000","size":5,"soft_fail":false,"target_va":5379474624,"va":5373140251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a456b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6338976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373139696,"raw_hex":"e8a0b96000","size":5,"soft_fail":false,"target_va":5379479216,"va":5373140235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1833586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373139696,"raw_hex":"e88e05e4ff","size":5,"soft_fail":false,"target_va":5371306912,"va":5373140493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$tg_package_integrity..verify_package_manifest..CheckOutcome$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6321398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373139696,"raw_hex":"e8f6746000","size":5,"soft_fail":false,"target_va":5379463136,"va":5373141733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29838850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373139696,"raw_hex":"e8024ec701","size":5,"soft_fail":false,"target_va":5402980128,"va":5373141273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4265300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373139696,"raw_hex":"e854154100","size":5,"soft_fail":false,"target_va":5377405072,"va":5373139767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":315,"record_offset":3228,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":315,"record_offset":3432,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":315,"record_offset":3284,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":315,"record_offset":3340,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":315,"record_offset":4048,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":315,"record_offset":1864,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":315,"record_offset":2296,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_direct] easy_parallel::Parallel::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"easy-parallel","version":"3.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/easy-parallel-3.2.0/src/lib.rs
//     evidence={"module":315,"record_offset":3200,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":17154,"source_line":85,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":315,"record_offset":1836,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":7565,"source_line":259,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":315,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":1672,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":315,"record_offset":2212,"parent_record_offset":2156,"ancestor_record_offset":1672,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":315,"record_offset":2916,"parent_record_offset":2888,"ancestor_record_offset":1672,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":315,"record_offset":2184,"parent_record_offset":2156,"ancestor_record_offset":1672,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":315,"record_offset":3984,"parent_record_offset":3952,"ancestor_record_offset":1672,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":315,"record_offset":2156,"parent_record_offset":2128,"ancestor_record_offset":1672,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":315,"record_offset":1988,"parent_record_offset":1940,"ancestor_record_offset":1672,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":315,"record_offset":1940,"parent_record_offset":1908,"ancestor_record_offset":1672,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":315,"record_offset":3952,"parent_record_offset":3920,"ancestor_record_offset":1672,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":315,"record_offset":4748,"parent_record_offset":4724,"ancestor_record_offset":1672,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":315,"record_offset":4076,"parent_record_offset":4048,"ancestor_record_offset":1672,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":315,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":1672,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":315,"record_offset":2016,"parent_record_offset":1940,"ancestor_record_offset":1672,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":315,"record_offset":4608,"parent_record_offset":4580,"ancestor_record_offset":1672,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":315,"record_offset":2080,"parent_record_offset":1940,"ancestor_record_offset":1672,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":315,"record_offset":3308,"parent_record_offset":3284,"ancestor_record_offset":1672,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":315,"record_offset":3392,"parent_record_offset":3340,"ancestor_record_offset":1672,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":315,"record_offset":4016,"parent_record_offset":3920,"ancestor_record_offset":1672,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":315,"record_offset":2044,"parent_record_offset":1940,"ancestor_record_offset":1672,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::verify_package_manifest::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":315,"record_offset":1908,"parent_record_offset":1864,"ancestor_record_offset":1672,"inlinee":17151,"source_line":26,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::verify_package_manifest::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":315,"record_offset":2376,"parent_record_offset":2324,"ancestor_record_offset":1672,"inlinee":17152,"source_line":29,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC674
// 首条位置: {"module":1002,"inlinee":50804}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC678
// 首条位置: {"module":1002,"inlinee":50808}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1002,"record_offset":20588,"parent_record_offset":20560,"ancestor_record_offset":20560,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1002,"record_offset":20612,"parent_record_offset":20588,"ancestor_record_offset":20560,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1002,"record_offset":20636,"parent_record_offset":20612,"ancestor_record_offset":20560,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1002,"record_offset":20660,"parent_record_offset":20636,"ancestor_record_offset":20560,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC677
// 首条位置: {"module":1002,"inlinee":50807}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1002,"record_offset":20412,"parent_record_offset":20384,"ancestor_record_offset":20384,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1002,"record_offset":20532,"parent_record_offset":20440,"ancestor_record_offset":20384,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1002,"record_offset":20612,"parent_record_offset":20588,"ancestor_record_offset":20384,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1002,"record_offset":20636,"parent_record_offset":20612,"ancestor_record_offset":20384,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1002,"record_offset":20660,"parent_record_offset":20636,"ancestor_record_offset":20384,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1002,"record_offset":20588,"parent_record_offset":20560,"ancestor_record_offset":20384,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":20476,"parent_record_offset":20440,"ancestor_record_offset":20384,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":20440,"parent_record_offset":20412,"ancestor_record_offset":20384,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1002,"record_offset":20500,"parent_record_offset":20476,"ancestor_record_offset":20384,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":1002,"record_offset":20560,"parent_record_offset":20440,"ancestor_record_offset":20384,"inlinee":50808,"source_line":17,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4308
// 首条位置: {"module":2362,"inlinee":17160}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC675
// 首条位置: {"module":1002,"inlinee":50805}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1002,"record_offset":20844,"parent_record_offset":19764,"ancestor_record_offset":19764,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1002,"record_offset":24424,"parent_record_offset":19764,"ancestor_record_offset":19764,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1002,"record_offset":24356,"parent_record_offset":19764,"ancestor_record_offset":19764,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1002,"record_offset":25152,"parent_record_offset":19764,"ancestor_record_offset":19764,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1002,"record_offset":19828,"parent_record_offset":19764,"ancestor_record_offset":19764,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1002,"record_offset":20872,"parent_record_offset":19764,"ancestor_record_offset":19764,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1002,"record_offset":24704,"parent_record_offset":24680,"ancestor_record_offset":19764,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1002,"record_offset":24680,"parent_record_offset":24616,"ancestor_record_offset":19764,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1002,"record_offset":24648,"parent_record_offset":24616,"ancestor_record_offset":19764,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1002,"record_offset":23060,"parent_record_offset":23028,"ancestor_record_offset":19764,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1002,"record_offset":24616,"parent_record_offset":24584,"ancestor_record_offset":19764,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1002,"record_offset":23028,"parent_record_offset":22996,"ancestor_record_offset":19764,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1002,"record_offset":20532,"parent_record_offset":20440,"ancestor_record_offset":19764,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1002,"record_offset":22996,"parent_record_offset":21752,"ancestor_record_offset":19764,"inlinee":11505,"source_line":2050,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1002,"record_offset":22236,"parent_record_offset":21752,"ancestor_record_offset":19764,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1002,"record_offset":24452,"parent_record_offset":24424,"ancestor_record_offset":19764,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1002,"record_offset":20612,"parent_record_offset":20588,"ancestor_record_offset":19764,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1002,"record_offset":25048,"parent_record_offset":25020,"ancestor_record_offset":19764,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1002,"record_offset":24476,"parent_record_offset":24452,"ancestor_record_offset":19764,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1002,"record_offset":23356,"parent_record_offset":23324,"ancestor_record_offset":19764,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1002,"record_offset":20636,"parent_record_offset":20612,"ancestor_record_offset":19764,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1002,"record_offset":20660,"parent_record_offset":20636,"ancestor_record_offset":19764,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1002,"record_offset":20588,"parent_record_offset":20560,"ancestor_record_offset":19764,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1002,"record_offset":25224,"parent_record_offset":25200,"ancestor_record_offset":19764,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1002,"record_offset":25200,"parent_record_offset":25176,"ancestor_record_offset":19764,"inlinee":6874,"source_line":126,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1002,"record_offset":25312,"parent_record_offset":25288,"ancestor_record_offset":19764,"inlinee":24928,"source_line":126,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1002,"record_offset":21664,"parent_record_offset":21620,"ancestor_record_offset":19764,"inlinee":7237,"source_line":232,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1002,"record_offset":20356,"parent_record_offset":19876,"ancestor_record_offset":19764,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1002,"record_offset":24316,"parent_record_offset":24292,"ancestor_record_offset":19764,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1002,"record_offset":25288,"parent_record_offset":25264,"ancestor_record_offset":19764,"inlinee":9849,"source_line":134,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":20080,"parent_record_offset":19964,"ancestor_record_offset":19764,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::ignore_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":21752,"parent_record_offset":21708,"ancestor_record_offset":19764,"inlinee":11504,"source_line":1023,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::next_char
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":22440,"parent_record_offset":22376,"ancestor_record_offset":19764,"inlinee":11397,"source_line":224,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_ident
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":22376,"parent_record_offset":21752,"ancestor_record_offset":19764,"inlinee":11396,"source_line":371,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":20956,"parent_record_offset":20912,"ancestor_record_offset":19764,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":19964,"parent_record_offset":19876,"ancestor_record_offset":19764,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":20004,"parent_record_offset":19964,"ancestor_record_offset":19764,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":19876,"parent_record_offset":19828,"ancestor_record_offset":19764,"inlinee":11544,"source_line":1963,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":20912,"parent_record_offset":20872,"ancestor_record_offset":19764,"inlinee":9858,"source_line":1996,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":20440,"parent_record_offset":20412,"ancestor_record_offset":19764,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1002,"record_offset":20412,"parent_record_offset":20384,"ancestor_record_offset":19764,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$7::deserialize_ignored_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":21708,"parent_record_offset":21664,"ancestor_record_offset":19764,"inlinee":11503,"source_line":1893,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1002,"record_offset":20104,"parent_record_offset":20080,"ancestor_record_offset":19764,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1002,"record_offset":22476,"parent_record_offset":22440,"ancestor_record_offset":19764,"inlinee":11398,"source_line":489,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1002,"record_offset":20036,"parent_record_offset":20004,"ancestor_record_offset":19764,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":1002,"record_offset":20560,"parent_record_offset":20440,"ancestor_record_offset":19764,"inlinee":50808,"source_line":17,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":1002,"record_offset":20384,"parent_record_offset":20356,"ancestor_record_offset":19764,"inlinee":50807,"source_line":17,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC679
// 首条位置: {"module":1002,"inlinee":50809}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1002,"record_offset":25488,"parent_record_offset":25436,"ancestor_record_offset":25436,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1002,"record_offset":26224,"parent_record_offset":25760,"ancestor_record_offset":25436,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1002,"record_offset":26248,"parent_record_offset":26224,"ancestor_record_offset":25436,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":25644,"parent_record_offset":25540,"ancestor_record_offset":25436,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":25540,"parent_record_offset":25512,"ancestor_record_offset":25436,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":25572,"parent_record_offset":25540,"ancestor_record_offset":25436,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1002,"record_offset":25512,"parent_record_offset":25488,"ancestor_record_offset":25436,"inlinee":11507,"source_line":1916,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1002,"record_offset":25668,"parent_record_offset":25644,"ancestor_record_offset":25436,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1002,"record_offset":25604,"parent_record_offset":25572,"ancestor_record_offset":25436,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/01002.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::verify_package_manifest::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42FF
// 首条位置: {"module":315,"inlinee":17151}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":315,"record_offset":1940,"parent_record_offset":1908,"ancestor_record_offset":1908,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":315,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":1908,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":315,"record_offset":2212,"parent_record_offset":2156,"ancestor_record_offset":1908,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":315,"record_offset":2184,"parent_record_offset":2156,"ancestor_record_offset":1908,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":315,"record_offset":2156,"parent_record_offset":2128,"ancestor_record_offset":1908,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":315,"record_offset":1988,"parent_record_offset":1940,"ancestor_record_offset":1908,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":315,"record_offset":2016,"parent_record_offset":1940,"ancestor_record_offset":1908,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":315,"record_offset":2080,"parent_record_offset":1940,"ancestor_record_offset":1908,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":315,"record_offset":2044,"parent_record_offset":1940,"ancestor_record_offset":1908,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::verify_package_manifest::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4300
// 首条位置: {"module":315,"inlinee":17152}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":315,"record_offset":2416,"parent_record_offset":2376,"ancestor_record_offset":2376,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":315,"record_offset":2768,"parent_record_offset":2376,"ancestor_record_offset":2376,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":315,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":2376,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":315,"record_offset":2692,"parent_record_offset":2636,"ancestor_record_offset":2376,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":315,"record_offset":2916,"parent_record_offset":2888,"ancestor_record_offset":2376,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":315,"record_offset":2664,"parent_record_offset":2636,"ancestor_record_offset":2376,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":315,"record_offset":2636,"parent_record_offset":2608,"ancestor_record_offset":2376,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":315,"record_offset":2468,"parent_record_offset":2416,"ancestor_record_offset":2376,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":315,"record_offset":2496,"parent_record_offset":2416,"ancestor_record_offset":2376,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":315,"record_offset":2560,"parent_record_offset":2416,"ancestor_record_offset":2376,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":315,"record_offset":2524,"parent_record_offset":2416,"ancestor_record_offset":2376,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00315.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::verify_package_manifest::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC681
// 首条位置: {"module":1003,"inlinee":50817}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1003,"record_offset":8216,"parent_record_offset":7876,"ancestor_record_offset":7876,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/01003.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1003,"record_offset":8564,"parent_record_offset":7876,"ancestor_record_offset":7876,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/01003.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1003,"record_offset":7992,"parent_record_offset":7876,"ancestor_record_offset":7876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01003.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1003,"record_offset":7964,"parent_record_offset":7876,"ancestor_record_offset":7876,"inlinee":7565,"source_line":259,"archive":"_pdb/A/modules/01003.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1003,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":7876,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01003.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1003,"record_offset":8144,"parent_record_offset":8088,"ancestor_record_offset":7876,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01003.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1003,"record_offset":8116,"parent_record_offset":8088,"ancestor_record_offset":7876,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01003.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1003,"record_offset":8088,"parent_record_offset":8064,"ancestor_record_offset":7876,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01003.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1003,"record_offset":8264,"parent_record_offset":8216,"ancestor_record_offset":7876,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/01003.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1003,"record_offset":8588,"parent_record_offset":8564,"ancestor_record_offset":7876,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/01003.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1003,"record_offset":8612,"parent_record_offset":8588,"ancestor_record_offset":7876,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/01003.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1003,"record_offset":8292,"parent_record_offset":8216,"ancestor_record_offset":7876,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/01003.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1003,"record_offset":8040,"parent_record_offset":8016,"ancestor_record_offset":7876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01003.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1003,"record_offset":8320,"parent_record_offset":8216,"ancestor_record_offset":7876,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01003.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1003,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":7876,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01003.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1003,"record_offset":8660,"parent_record_offset":8636,"ancestor_record_offset":7876,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01003.jsonl.gz"}
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
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_package_integrity::PackageManifest
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tg_package_integrity::PackageManifest>", "constructor": "core::ptr::drop_in_place", "arguments": ["tg_package_integrity::PackageManifest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_slice<tg_package_integrity::PackageManifest>", "constructor": "serde_json::de::from_slice", "arguments": ["tg_package_integrity::PackageManifest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tg_package_integrity::PackageManifest,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["tg_package_integrity::PackageManifest", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tg_package_integrity::PackageManifest,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tg_package_integrity::PackageManifest,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tg_package_integrity::PackageManifest,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tg_package_integrity::PackageManifest,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_package_integrity::PackageVerificationError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tg_package_integrity::PackageVerificationError>", "constructor": "enum2$", "arguments": ["tg_package_integrity::PackageVerificationError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<tg_package_integrity::PackageVerificationError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<tg_package_integrity::PackageVerificationError> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<tg_package_integrity::PackageVerificationError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_package_integrity::verify_package_manifest::CheckOutcome
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 43
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>", "constructor": "enum2$", "arguments": ["tg_package_integrity::verify_package_manifest::CheckOutcome"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "assoc$", "arguments": ["Output", "enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::ops::function::FnOnce", "arguments": ["tuple$<>", "assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>", "constructor": "dyn$", "arguments": ["core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "core::marker::Send"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::Parallel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "easy_parallel::Parallel", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<easy_parallel::Parallel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["easy_parallel::Parallel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Sender<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpsc::Sender", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Sender<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Sender<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpsc::Receiver", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::SendError<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpsc::SendError", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::SendError<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::SendError<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Packet<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::Packet", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::zero::Packet<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::zero::Packet<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::list::Channel", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::list::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::array::Channel", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::array::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::Parallel::run<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "easy_parallel::Parallel::run", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::Parallel::each<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$2>", "constructor": "easy_parallel::Parallel::each", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>", "tuple$<alloc::string::String,alloc::string::String>", "alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>", "tg_package_integrity::verify_package_manifest::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::impl$0::finish::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "easy_parallel::impl$0::finish::closure_env$0", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>", "alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>", "enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<easy_parallel::impl$0::finish::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["easy_parallel::impl$0::finish::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$2>", "constructor": "easy_parallel::impl$0::each::closure_env$0", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>", "tuple$<alloc::string::String,alloc::string::String>", "alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>", "tg_package_integrity::verify_package_manifest::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$2>,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$2>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$2> >", "constructor": "core::ptr::drop_in_place", "arguments": ["easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<easy_parallel::impl$0::finish::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["easy_parallel::impl$0::finish::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_senders<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::list::Channel::disconnect_senders", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::recv<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::list::Channel::recv", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::impl$3::recv::closure$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::list::impl$3::recv::closure$1", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::impl$15::drop::closure_env$2", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::recv<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::array::Channel::recv", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::recv::closure$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::array::impl$1::recv::closure$1", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::send::closure$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::array::impl$1::send::closure$0", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global> >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::panicking::try<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,core::panic::unwind_safe::AssertUnwindSafe<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global> > >", "constructor": "std::panicking::try", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>", "core::panic::unwind_safe::AssertUnwindSafe<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::disconnect<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::Channel::disconnect", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::recv<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::Channel::recv", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::recv::closure$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::impl$3::recv::closure$1", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::recv::closure_env$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::impl$3::recv::closure_env$1", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::recv::closure_env$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::impl$18::fmt<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpsc::impl$18::fmt", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize
// 信息 ID: 547a0ff8db7f2fb693820247
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::from_slice<tg_package_integrity::PackageManifest>; module=1002, inline_record=496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 85d3a74200d93f35ff3c8fe2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor>; module=1002, inline_record=20560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 45d3d53a9d5794172250f9e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor>; module=1002, inline_record=20384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: bfdcc88662af0a8f88f5205e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<tg_package_integrity::_::impl$0::deserialize::__Visitor>; module=2362, inline_record=420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 3c79ddf646a6f6c005a4e3b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor>; module=1002, inline_record=19764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 72e1ccc66bbefeb989ac1868
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor>; module=1002, inline_record=25436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::verify_package_manifest
// 信息 ID: 0538a357d1bc6ff8b6457189
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x71c8 -> 0x439af0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::integrity_report_main::verify_package_integrity_and_show_report @ 0x307f35 -> 0x439af0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::verify_package_manifest::closure$0
// 信息 ID: b05e6329f67714f0b4f845f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_package_integrity::verify_package_manifest", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_package_integrity::verify_package_manifest; module=315, inline_record=1908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::verify_package_manifest::closure$1
// 信息 ID: 245ffebcb7c0851a1e6c5729
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_package_integrity::verify_package_manifest", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_package_integrity::verify_package_manifest; module=315, inline_record=2376; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::verify_package_manifest::closure$2
// 信息 ID: 562942e0957ee6d1ef15d465
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_package_integrity::verify_package_manifest", "captures": "unknown"}
// [闭包环境类型引用] tg_package_integrity::verify_package_manifest::closure_env$2 <- easy_parallel::Parallel::each<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$2>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_package_integrity::verify_package_manifest::closure_env$2 <- core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$2>,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_package_integrity::verify_package_manifest::closure_env$2 <- core::ptr::drop_in_place<easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$2>,tuple$<> >; module=1003, inline_record=7876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
