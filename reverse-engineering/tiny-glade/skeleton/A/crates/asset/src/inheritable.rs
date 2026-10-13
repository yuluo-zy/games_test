// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/asset/src/inheritable.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5c19e731c2af936e8d8d2aacd637ee1e122731dfae6a9c2a530a2b3808f18ccf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: inheritable.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 398, 1451, 3912, 3976, 4398, 4431, 4432, 4433, 4434, 4572, 4585, 4596, 4632
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use ron; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$7::clone
// 记录形式: inline_instance, inlinee_declaration; instances=81
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B621
// 首条位置: {"module":3912,"inlinee":112161}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::impl$17::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":2488,"parent_record_offset":2440,"ancestor_record_offset":2440,"inlinee":112740,"source_line":71,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":3560,"parent_record_offset":3524,"ancestor_record_offset":3524,"inlinee":112748,"source_line":185,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$38::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":3368,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":112744,"source_line":226,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::impl$0::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":3912,"record_offset":4432,"parent_record_offset":4404,"ancestor_record_offset":4404,"inlinee":112163,"source_line":4,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::impl$14::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":3912,"record_offset":4600,"parent_record_offset":4572,"ancestor_record_offset":4572,"inlinee":112167,"source_line":19,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":3976,"record_offset":1652,"parent_record_offset":1624,"ancestor_record_offset":1624,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3912,"record_offset":4328,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3976,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":2440,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3976,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":2440,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3976,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":2440,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=24
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3976,"record_offset":2540,"parent_record_offset":2488,"ancestor_record_offset":2440,"inlinee":112161,"source_line":13,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":2640,"parent_record_offset":2608,"ancestor_record_offset":2440,"inlinee":66970,"source_line":108,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$26::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":3244,"parent_record_offset":3212,"ancestor_record_offset":3180,"inlinee":112742,"source_line":114,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$31::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":3488,"parent_record_offset":3448,"ancestor_record_offset":3412,"inlinee":112746,"source_line":163,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":3560,"parent_record_offset":3524,"ancestor_record_offset":3412,"inlinee":112748,"source_line":185,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$38::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":3368,"parent_record_offset":3340,"ancestor_record_offset":3180,"inlinee":112744,"source_line":226,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3976,"record_offset":2608,"parent_record_offset":2576,"ancestor_record_offset":2440,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3976,"record_offset":2680,"parent_record_offset":2488,"ancestor_record_offset":2440,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::Inheritable::inherit_shallow
// 记录形式: inline_instance, inlinee_declaration; instances=46
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B61F
// 首条位置: {"module":3912,"inlinee":112159}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=17
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3912,"record_offset":4300,"parent_record_offset":4268,"ancestor_record_offset":4268,"inlinee":112161,"source_line":13,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4596,"record_offset":524,"parent_record_offset":496,"ancestor_record_offset":436,"inlinee":66970,"source_line":108,"archive":"_pdb/A/modules/04596.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$38::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4585,"record_offset":1148,"parent_record_offset":1120,"ancestor_record_offset":1092,"inlinee":112744,"source_line":226,"archive":"_pdb/A/modules/04585.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$0::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":3912,"record_offset":4432,"parent_record_offset":4404,"ancestor_record_offset":4364,"inlinee":112163,"source_line":4,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$14::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":3912,"record_offset":4600,"parent_record_offset":4572,"ancestor_record_offset":4540,"inlinee":112167,"source_line":19,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":3976,"record_offset":1652,"parent_record_offset":1624,"ancestor_record_offset":1596,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3912,"record_offset":4328,"parent_record_offset":4300,"ancestor_record_offset":4268,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::Inheritable::as_concrete
// 记录形式: inline_instance, inlinee_declaration; instances=34
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x595C
// 首条位置: {"module":1451,"inlinee":22876}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":17848,"parent_record_offset":17808,"ancestor_record_offset":17808,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":17876,"parent_record_offset":17808,"ancestor_record_offset":17808,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::Inheritable::into_concrete
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D8CB
// 首条位置: {"module":4398,"inlinee":121035}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4398,"record_offset":432,"parent_record_offset":392,"ancestor_record_offset":392,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/04398.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4398,"record_offset":460,"parent_record_offset":392,"ancestor_record_offset":392,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/04398.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$1::inherit_from<asset::prefab::config::AudioTriggerConfig>
// 记录形式: compiled_procedure; instances=1
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4432,"record_offset":5292,"rva":32096512,"va":5400805632}
// 编译实例 RVAs: 0x1e9c100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31173437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400805632,"raw_hex":"e8c35424fe","size":5,"soft_fail":false,"target_va":5369632448,"va":5400805880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28056492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400805632,"raw_hex":"e854e453fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400806199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2172753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400805632,"raw_hex":"e851272100","size":5,"soft_fail":false,"target_va":5402979152,"va":5400806394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2172569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400805632,"raw_hex":"e899262100","size":5,"soft_fail":false,"target_va":5402978944,"va":5400806370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":5472,"parent_record_offset":5292,"ancestor_record_offset":5292,"inlinee":22876,"source_line":37,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":5792,"parent_record_offset":5292,"ancestor_record_offset":5292,"inlinee":112161,"source_line":13,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$10::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":5576,"parent_record_offset":5292,"ancestor_record_offset":5292,"inlinee":121285,"source_line":198,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4432,"record_offset":5932,"parent_record_offset":5292,"ancestor_record_offset":5292,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4432,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":5292,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4432,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":5292,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4432,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":5292,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":5616,"parent_record_offset":5576,"ancestor_record_offset":5292,"inlinee":112159,"source_line":30,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":5684,"parent_record_offset":5652,"ancestor_record_offset":5292,"inlinee":112161,"source_line":13,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":5824,"parent_record_offset":5792,"ancestor_record_offset":5292,"inlinee":112748,"source_line":185,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4432,"record_offset":5544,"parent_record_offset":5472,"ancestor_record_offset":5292,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4432,"record_offset":5516,"parent_record_offset":5472,"ancestor_record_offset":5292,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4432,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":5292,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$1::inherit_from<asset::prefab::config::PrefabRenderConfig>
// 记录形式: compiled_procedure; instances=1
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4432,"record_offset":2544,"rva":32094560,"va":5400803680}
// 编译实例 RVAs: 0x1e9b960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31171487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400803680,"raw_hex":"e8615c24fe","size":5,"soft_fail":false,"target_va":5369632448,"va":5400803930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28054513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400803680,"raw_hex":"e80fec53fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400804220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2174650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400803680,"raw_hex":"e8ba2e2100","size":5,"soft_fail":false,"target_va":5402979152,"va":5400804497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2174466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400803680,"raw_hex":"e8022e2100","size":5,"soft_fail":false,"target_va":5402978944,"va":5400804473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142058160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t1820535","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5400803680,"raw_hex":"e977c71b00","size":5,"soft_fail":false,"target_va":5402624352,"va":5400803812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":2720,"parent_record_offset":2544,"ancestor_record_offset":2544,"inlinee":22876,"source_line":37,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":2824,"parent_record_offset":2544,"ancestor_record_offset":2544,"inlinee":112161,"source_line":13,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4432,"record_offset":3532,"parent_record_offset":2544,"ancestor_record_offset":2544,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4432,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":2544,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4432,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":2544,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4432,"record_offset":3180,"parent_record_offset":3156,"ancestor_record_offset":2544,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":2904,"parent_record_offset":2856,"ancestor_record_offset":2544,"inlinee":112161,"source_line":13,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":2856,"parent_record_offset":2824,"ancestor_record_offset":2544,"inlinee":112740,"source_line":71,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":2996,"parent_record_offset":2968,"ancestor_record_offset":2544,"inlinee":66970,"source_line":108,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4432,"record_offset":2792,"parent_record_offset":2720,"ancestor_record_offset":2544,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4432,"record_offset":2764,"parent_record_offset":2720,"ancestor_record_offset":2544,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4432,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":2544,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4432,"record_offset":3032,"parent_record_offset":2856,"ancestor_record_offset":2544,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4432,"record_offset":4256,"rva":32095616,"va":5400804736}
// 编译实例 RVAs: 0x1e9bd80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31172692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400804736,"raw_hex":"e8ac5724fe","size":5,"soft_fail":false,"target_va":5369632448,"va":5400805135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28055558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400804736,"raw_hex":"e8fae753fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400805265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e9c100
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400804736,"raw_hex":"e8f7020000","size":5,"soft_fail":false,"target_va":5400805632,"va":5400804868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2173634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400804736,"raw_hex":"e8c22a2100","size":5,"soft_fail":false,"target_va":5402979152,"va":5400805513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2173450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400804736,"raw_hex":"e80a2a2100","size":5,"soft_fail":false,"target_va":5402978944,"va":5400805489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$asset..prefab..config..PrefabAudioConfig$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2068783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400804736,"raw_hex":"e8d16ee0ff","size":5,"soft_fail":false,"target_va":5398736832,"va":5400805610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":4468,"parent_record_offset":4256,"ancestor_record_offset":4256,"inlinee":22876,"source_line":37,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$3::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":4572,"parent_record_offset":4256,"ancestor_record_offset":4256,"inlinee":121279,"source_line":99,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":4704,"parent_record_offset":4256,"ancestor_record_offset":4256,"inlinee":112161,"source_line":13,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4432,"record_offset":4948,"parent_record_offset":4256,"ancestor_record_offset":4256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4432,"record_offset":5216,"parent_record_offset":5192,"ancestor_record_offset":4256,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4432,"record_offset":5192,"parent_record_offset":5168,"ancestor_record_offset":4256,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4432,"record_offset":5168,"parent_record_offset":5144,"ancestor_record_offset":4256,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":4796,"parent_record_offset":4768,"ancestor_record_offset":4256,"inlinee":112161,"source_line":13,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":4768,"parent_record_offset":4736,"ancestor_record_offset":4256,"inlinee":112746,"source_line":163,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":4828,"parent_record_offset":4796,"ancestor_record_offset":4256,"inlinee":112748,"source_line":185,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$8::inherit_from
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":4616,"parent_record_offset":4572,"ancestor_record_offset":4256,"inlinee":121281,"source_line":170,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$9::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":4644,"parent_record_offset":4572,"ancestor_record_offset":4256,"inlinee":121283,"source_line":177,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4432,"record_offset":4540,"parent_record_offset":4468,"ancestor_record_offset":4256,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4432,"record_offset":4512,"parent_record_offset":4468,"ancestor_record_offset":4256,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4432,"record_offset":4736,"parent_record_offset":4704,"ancestor_record_offset":4256,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4432,"record_offset":4976,"parent_record_offset":4948,"ancestor_record_offset":4256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4432,"record_offset":2016,"rva":32094000,"va":5400803120}
// 编译实例 RVAs: 0x1e9b730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2175535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400803120,"raw_hex":"e82f322100","size":5,"soft_fail":false,"target_va":5402979152,"va":5400803612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2175351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400803120,"raw_hex":"e877312100","size":5,"soft_fail":false,"target_va":5402978944,"va":5400803588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142052e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t1799889","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5400803120,"raw_hex":"e9d1761b00","size":5,"soft_fail":false,"target_va":5402603088,"va":5400803194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":2216,"parent_record_offset":2016,"ancestor_record_offset":2016,"inlinee":22876,"source_line":37,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":2320,"parent_record_offset":2016,"ancestor_record_offset":2016,"inlinee":112161,"source_line":13,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":2412,"parent_record_offset":2376,"ancestor_record_offset":2016,"inlinee":112161,"source_line":13,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":2376,"parent_record_offset":2348,"ancestor_record_offset":2016,"inlinee":112742,"source_line":114,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$38::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":2496,"parent_record_offset":2468,"ancestor_record_offset":2016,"inlinee":112744,"source_line":226,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4432,"record_offset":2288,"parent_record_offset":2216,"ancestor_record_offset":2016,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4432,"record_offset":2260,"parent_record_offset":2216,"ancestor_record_offset":2016,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4432,"record_offset":2348,"parent_record_offset":2320,"ancestor_record_offset":2016,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$3::inherit_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D9BF
// 首条位置: {"module":4432,"inlinee":121279}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::impl$8::inherit_from
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":4616,"parent_record_offset":4572,"ancestor_record_offset":4572,"inlinee":121281,"source_line":170,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$9::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":4644,"parent_record_offset":4572,"ancestor_record_offset":4572,"inlinee":121283,"source_line":177,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$3::inherit_from<asset::prefab::config::PrefabRtConfig>
// 记录形式: compiled_procedure; instances=1
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4585,"record_offset":740,"rva":33893968,"va":5402603088}
// 编译实例 RVAs: 0x2052e50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::impl$6::inherit_from
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4585,"record_offset":904,"parent_record_offset":740,"ancestor_record_offset":740,"inlinee":122935,"source_line":130,"archive":"_pdb/A/modules/04585.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4585,"record_offset":948,"parent_record_offset":904,"ancestor_record_offset":740,"inlinee":112159,"source_line":30,"archive":"_pdb/A/modules/04585.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4585,"record_offset":1008,"parent_record_offset":980,"ancestor_record_offset":740,"inlinee":112161,"source_line":13,"archive":"_pdb/A/modules/04585.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$38::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4585,"record_offset":1148,"parent_record_offset":1120,"ancestor_record_offset":740,"inlinee":112744,"source_line":226,"archive":"_pdb/A/modules/04585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>
// 记录形式: compiled_procedure; instances=1
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3976,"record_offset":244,"rva":30031344,"va":5398740464}
// 编译实例 RVAs: 0x1ca3df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=5
//     target_aliases=4 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29108203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398740464,"raw_hex":"e815d843fe","size":5,"soft_fail":false,"target_va":5369632448,"va":5398740646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25991144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398740464,"raw_hex":"e8186873fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398740851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ca11a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398740464,"raw_hex":"e8e2d2ffff","size":5,"soft_fail":false,"target_va":5398729120,"va":5398740665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e9b730
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2062325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398740464,"raw_hex":"e8f5771f00","size":5,"soft_fail":false,"target_va":5400803120,"va":5398740790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e9b960
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2062899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398740464,"raw_hex":"e8337a1f00","size":5,"soft_fail":false,"target_va":5400803680,"va":5398740776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e9bd80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2063922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398740464,"raw_hex":"e8327e1f00","size":5,"soft_fail":false,"target_va":5400804736,"va":5398740809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$asset..prefab..config..PrefabPartConfig$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398740464,"raw_hex":"e88be5ffff","size":5,"soft_fail":false,"target_va":5398736736,"va":5398743504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$asset..prefab..config..PrefabRenderConfig$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398740464,"raw_hex":"e885e7ffff","size":5,"soft_fail":false,"target_va":5398736896,"va":5398743158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::prefab::config::impl$2::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":6100,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":112750,"source_line":44,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$3::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":6128,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":112735,"source_line":55,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3976,"record_offset":6196,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::and_modify
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3976,"record_offset":1496,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":21450,"source_line":2741,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3976,"record_offset":1952,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":12248,"source_line":2663,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3976,"record_offset":1440,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3976,"record_offset":5084,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":6439,"source_line":456,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3976,"record_offset":468,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3976,"record_offset":820,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$56::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3976,"record_offset":5460,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":8349,"source_line":2427,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3976,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":244,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3976,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":244,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3976,"record_offset":4972,"parent_record_offset":4944,"ancestor_record_offset":244,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3976,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":244,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3976,"record_offset":1596,"parent_record_offset":1560,"ancestor_record_offset":244,"inlinee":112159,"source_line":30,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$5::inherit_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3976,"record_offset":1532,"parent_record_offset":1496,"ancestor_record_offset":244,"inlinee":112733,"source_line":131,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$5::inherit_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3976,"record_offset":2272,"parent_record_offset":1952,"ancestor_record_offset":244,"inlinee":112736,"source_line":132,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=14
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3976,"record_offset":1624,"parent_record_offset":1596,"ancestor_record_offset":244,"inlinee":112161,"source_line":13,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$14::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":2312,"parent_record_offset":2272,"ancestor_record_offset":244,"inlinee":112738,"source_line":27,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":2488,"parent_record_offset":2440,"ancestor_record_offset":244,"inlinee":112740,"source_line":71,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":2640,"parent_record_offset":2608,"ancestor_record_offset":244,"inlinee":66970,"source_line":108,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":3244,"parent_record_offset":3212,"ancestor_record_offset":244,"inlinee":112742,"source_line":114,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":3488,"parent_record_offset":3448,"ancestor_record_offset":244,"inlinee":112746,"source_line":163,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":3560,"parent_record_offset":3524,"ancestor_record_offset":244,"inlinee":112748,"source_line":185,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$38::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":3368,"parent_record_offset":3340,"ancestor_record_offset":244,"inlinee":112744,"source_line":226,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$3::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":1560,"parent_record_offset":1532,"ancestor_record_offset":244,"inlinee":112735,"source_line":55,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":3976,"record_offset":1652,"parent_record_offset":1624,"ancestor_record_offset":244,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3976,"record_offset":672,"parent_record_offset":644,"ancestor_record_offset":244,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3976,"record_offset":3960,"parent_record_offset":3932,"ancestor_record_offset":244,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3976,"record_offset":732,"parent_record_offset":700,"ancestor_record_offset":244,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3976,"record_offset":3988,"parent_record_offset":3960,"ancestor_record_offset":244,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3976,"record_offset":4544,"parent_record_offset":4508,"ancestor_record_offset":244,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3976,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":244,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3976,"record_offset":2608,"parent_record_offset":2576,"ancestor_record_offset":244,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3976,"record_offset":768,"parent_record_offset":620,"ancestor_record_offset":244,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=52
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3976,"record_offset":1696,"parent_record_offset":1496,"ancestor_record_offset":244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3976,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":244,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3976,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":244,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3976,"record_offset":4752,"parent_record_offset":4728,"ancestor_record_offset":244,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3976,"record_offset":4776,"parent_record_offset":4752,"ancestor_record_offset":244,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3976,"record_offset":516,"parent_record_offset":492,"ancestor_record_offset":244,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3976,"record_offset":5140,"parent_record_offset":5112,"ancestor_record_offset":244,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3976,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":244,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3976,"record_offset":5516,"parent_record_offset":5488,"ancestor_record_offset":244,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":4640,"parent_record_offset":4616,"ancestor_record_offset":244,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":1228,"parent_record_offset":900,"ancestor_record_offset":244,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":4728,"parent_record_offset":3772,"ancestor_record_offset":244,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":4164,"parent_record_offset":3848,"ancestor_record_offset":244,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":620,"parent_record_offset":564,"ancestor_record_offset":244,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":900,"parent_record_offset":868,"ancestor_record_offset":244,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":244,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":244,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":4616,"parent_record_offset":3772,"ancestor_record_offset":244,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":592,"parent_record_offset":564,"ancestor_record_offset":244,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":3848,"parent_record_offset":3816,"ancestor_record_offset":244,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":4028,"parent_record_offset":3848,"ancestor_record_offset":244,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":4192,"parent_record_offset":3848,"ancestor_record_offset":244,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":4224,"parent_record_offset":4192,"ancestor_record_offset":244,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":244,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":3816,"parent_record_offset":3772,"ancestor_record_offset":244,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":3904,"parent_record_offset":3848,"ancestor_record_offset":244,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":4508,"parent_record_offset":4484,"ancestor_record_offset":244,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":4484,"parent_record_offset":3816,"ancestor_record_offset":244,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3976,"record_offset":976,"parent_record_offset":944,"ancestor_record_offset":244,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3976,"record_offset":1008,"parent_record_offset":976,"ancestor_record_offset":244,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3976,"record_offset":1068,"parent_record_offset":944,"ancestor_record_offset":244,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3976,"record_offset":944,"parent_record_offset":900,"ancestor_record_offset":244,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":244,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":244,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3976,"record_offset":4588,"parent_record_offset":3772,"ancestor_record_offset":244,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3976,"record_offset":3932,"parent_record_offset":3848,"ancestor_record_offset":244,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3976,"record_offset":644,"parent_record_offset":620,"ancestor_record_offset":244,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3976,"record_offset":700,"parent_record_offset":644,"ancestor_record_offset":244,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::into_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":3976,"record_offset":2016,"parent_record_offset":1992,"ancestor_record_offset":244,"inlinee":6550,"source_line":412,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":3976,"record_offset":3748,"parent_record_offset":3724,"ancestor_record_offset":244,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3976,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":244,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3976,"record_offset":5112,"parent_record_offset":5084,"ancestor_record_offset":244,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::into_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3976,"record_offset":1992,"parent_record_offset":1952,"ancestor_record_offset":244,"inlinee":6548,"source_line":2927,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3976,"record_offset":3724,"parent_record_offset":1952,"ancestor_record_offset":244,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3976,"record_offset":5488,"parent_record_offset":5460,"ancestor_record_offset":244,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3976,"record_offset":1464,"parent_record_offset":1440,"ancestor_record_offset":244,"inlinee":15015,"source_line":3182,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$5::inherit_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B85D
// 首条位置: {"module":3976,"inlinee":112733}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::impl$3::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":1560,"parent_record_offset":1532,"ancestor_record_offset":1532,"inlinee":112735,"source_line":55,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3976,"record_offset":1596,"parent_record_offset":1560,"ancestor_record_offset":1532,"inlinee":112159,"source_line":30,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3976,"record_offset":1624,"parent_record_offset":1596,"ancestor_record_offset":1532,"inlinee":112161,"source_line":13,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":3976,"record_offset":1652,"parent_record_offset":1624,"ancestor_record_offset":1532,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$5::inherit_from::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B860
// 首条位置: {"module":3976,"inlinee":112736}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::impl$14::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":2312,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":112738,"source_line":27,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3976,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":2272,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3976,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":2272,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3976,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":2272,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=13
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3976,"record_offset":2380,"parent_record_offset":2312,"ancestor_record_offset":2272,"inlinee":112161,"source_line":13,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":2488,"parent_record_offset":2440,"ancestor_record_offset":2272,"inlinee":112740,"source_line":71,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":2640,"parent_record_offset":2608,"ancestor_record_offset":2272,"inlinee":66970,"source_line":108,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":3244,"parent_record_offset":3212,"ancestor_record_offset":2272,"inlinee":112742,"source_line":114,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":3488,"parent_record_offset":3448,"ancestor_record_offset":2272,"inlinee":112746,"source_line":163,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":3560,"parent_record_offset":3524,"ancestor_record_offset":2272,"inlinee":112748,"source_line":185,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$38::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3976,"record_offset":3368,"parent_record_offset":3340,"ancestor_record_offset":2272,"inlinee":112744,"source_line":226,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":3976,"record_offset":2408,"parent_record_offset":2380,"ancestor_record_offset":2272,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3976,"record_offset":2608,"parent_record_offset":2576,"ancestor_record_offset":2272,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3976,"record_offset":2680,"parent_record_offset":2488,"ancestor_record_offset":2272,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03976.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable
// 记录形式: inline_instance, inlinee_declaration; instances=53
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D9AB
// 首条位置: {"module":4431,"inlinee":121259}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4431,"record_offset":43088,"parent_record_offset":43064,"ancestor_record_offset":43064,"inlinee":112761,"source_line":71,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4431,"record_offset":51788,"parent_record_offset":51764,"ancestor_record_offset":51764,"inlinee":112265,"source_line":4,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4431,"record_offset":51844,"parent_record_offset":51764,"ancestor_record_offset":51764,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4572,"record_offset":8908,"parent_record_offset":8812,"ancestor_record_offset":8812,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/04572.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4431,"record_offset":41064,"parent_record_offset":41040,"ancestor_record_offset":41040,"inlinee":6847,"source_line":72,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4431,"record_offset":34692,"parent_record_offset":34652,"ancestor_record_offset":34652,"inlinee":6890,"source_line":700,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$99::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4572,"record_offset":11976,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":6860,"source_line":1338,"archive":"_pdb/A/modules/04572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4431,"record_offset":35308,"parent_record_offset":35284,"ancestor_record_offset":34652,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4431,"record_offset":35332,"parent_record_offset":35308,"ancestor_record_offset":34652,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4431,"record_offset":35284,"parent_record_offset":35248,"ancestor_record_offset":34652,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4431,"record_offset":35248,"parent_record_offset":35220,"ancestor_record_offset":34652,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4431,"record_offset":35220,"parent_record_offset":35192,"ancestor_record_offset":34652,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4431,"record_offset":35404,"parent_record_offset":35220,"ancestor_record_offset":34652,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4431,"record_offset":35152,"parent_record_offset":35116,"ancestor_record_offset":34652,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4431,"record_offset":35116,"parent_record_offset":35080,"ancestor_record_offset":34652,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4431,"record_offset":35044,"parent_record_offset":35008,"ancestor_record_offset":34652,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4431,"record_offset":35080,"parent_record_offset":35044,"ancestor_record_offset":34652,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4431,"record_offset":35008,"parent_record_offset":34972,"ancestor_record_offset":34652,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":4431,"record_offset":34972,"parent_record_offset":34936,"ancestor_record_offset":34652,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4431,"record_offset":35192,"parent_record_offset":35152,"ancestor_record_offset":34652,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4431,"record_offset":35468,"parent_record_offset":35444,"ancestor_record_offset":34652,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4431,"record_offset":35444,"parent_record_offset":35152,"ancestor_record_offset":34652,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4431,"record_offset":35360,"parent_record_offset":35332,"ancestor_record_offset":34652,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4431,"record_offset":34860,"parent_record_offset":34812,"ancestor_record_offset":34652,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_bool
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":4431,"record_offset":41112,"parent_record_offset":41088,"ancestor_record_offset":41040,"inlinee":9498,"source_line":345,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":4431,"record_offset":34812,"parent_record_offset":34772,"ancestor_record_offset":34652,"inlinee":9499,"source_line":445,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":4431,"record_offset":34772,"parent_record_offset":34732,"ancestor_record_offset":34652,"inlinee":7915,"source_line":455,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_bool
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4431,"record_offset":41088,"parent_record_offset":41064,"ancestor_record_offset":41040,"inlinee":9518,"source_line":39,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4431,"record_offset":51812,"parent_record_offset":51788,"ancestor_record_offset":51764,"inlinee":8726,"source_line":233,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_string
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4431,"record_offset":34732,"parent_record_offset":34692,"ancestor_record_offset":34652,"inlinee":7913,"source_line":139,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4431,"record_offset":43112,"parent_record_offset":43088,"ancestor_record_offset":43064,"inlinee":8724,"source_line":221,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4431,"record_offset":34900,"parent_record_offset":34812,"ancestor_record_offset":34652,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$6::visit_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4431,"record_offset":34936,"parent_record_offset":34900,"ancestor_record_offset":34652,"inlinee":7924,"source_line":603,"archive":"_pdb/A/modules/04431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,array$<u32,2> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4432,"record_offset":8008,"rva":32099904,"va":5400809024}
// 编译实例 RVAs: 0x1e9ce40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2782046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809024,"raw_hex":"e85e732a00","size":5,"soft_fail":false,"target_va":5403591824,"va":5400809773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e90910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-50535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809024,"raw_hex":"e8993affff","size":5,"soft_fail":false,"target_va":5400758544,"va":5400809074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ef1030
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t344387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809024,"raw_hex":"e843410500","size":5,"soft_fail":false,"target_va":5401153584,"va":5400809192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809024,"raw_hex":"e8e9e3ffff","size":5,"soft_fail":false,"target_va":5400802400,"va":5400809586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809024,"raw_hex":"e8a6e3ffff","size":5,"soft_fail":false,"target_va":5400802624,"va":5400809877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":8204,"parent_record_offset":8008,"ancestor_record_offset":8008,"inlinee":121287,"source_line":175,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":8808,"parent_record_offset":8008,"ancestor_record_offset":8008,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":8844,"parent_record_offset":8008,"ancestor_record_offset":8008,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4432,"record_offset":8684,"parent_record_offset":8204,"ancestor_record_offset":8008,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":8572,"parent_record_offset":8544,"ancestor_record_offset":8008,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":8320,"parent_record_offset":8204,"ancestor_record_offset":8008,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":8620,"parent_record_offset":8204,"ancestor_record_offset":8008,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4432,"record_offset":8284,"parent_record_offset":8260,"ancestor_record_offset":8008,"inlinee":30280,"source_line":32,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4432,"record_offset":8236,"parent_record_offset":8204,"ancestor_record_offset":8008,"inlinee":7227,"source_line":294,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_option
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4432,"record_offset":8388,"parent_record_offset":8356,"ancestor_record_offset":8008,"inlinee":121288,"source_line":1893,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4432,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":8008,"inlinee":9869,"source_line":1223,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4432,"record_offset":8432,"parent_record_offset":8388,"ancestor_record_offset":8008,"inlinee":9802,"source_line":912,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4432,"record_offset":8356,"parent_record_offset":8204,"ancestor_record_offset":8008,"inlinee":7231,"source_line":931,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$96::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4432,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":8008,"inlinee":10271,"source_line":1338,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabAudioConfig>
// 记录形式: compiled_procedure; instances=1
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4432,"record_offset":8876,"rva":32100992,"va":5400810112}
// 编译实例 RVAs: 0x1e9d280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2780667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400810112,"raw_hex":"e8fb6d2a00","size":5,"soft_fail":false,"target_va":5403591824,"va":5400811152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e90910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-51618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400810112,"raw_hex":"e85e36ffff","size":5,"soft_fail":false,"target_va":5400758544,"va":5400810157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ef1610
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t344780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400810112,"raw_hex":"e8cc420500","size":5,"soft_fail":false,"target_va":5401155088,"va":5400810303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400810112,"raw_hex":"e8dcdfffff","size":5,"soft_fail":false,"target_va":5400802400,"va":5400810623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-8313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400810112,"raw_hex":"e887dfffff","size":5,"soft_fail":false,"target_va":5400802624,"va":5400810932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":9100,"parent_record_offset":8876,"ancestor_record_offset":8876,"inlinee":121287,"source_line":175,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":9716,"parent_record_offset":8876,"ancestor_record_offset":8876,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":9760,"parent_record_offset":8876,"ancestor_record_offset":8876,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":9356,"parent_record_offset":9332,"ancestor_record_offset":8876,"inlinee":112761,"source_line":163,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4432,"record_offset":9592,"parent_record_offset":9100,"ancestor_record_offset":8876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":9472,"parent_record_offset":9444,"ancestor_record_offset":8876,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":9220,"parent_record_offset":9100,"ancestor_record_offset":8876,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":9524,"parent_record_offset":9100,"ancestor_record_offset":8876,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4432,"record_offset":9184,"parent_record_offset":9160,"ancestor_record_offset":8876,"inlinee":30280,"source_line":32,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4432,"record_offset":9136,"parent_record_offset":9100,"ancestor_record_offset":8876,"inlinee":7227,"source_line":294,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_option
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4432,"record_offset":9288,"parent_record_offset":9256,"ancestor_record_offset":8876,"inlinee":121288,"source_line":1893,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4432,"record_offset":9160,"parent_record_offset":9136,"ancestor_record_offset":8876,"inlinee":9869,"source_line":1223,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4432,"record_offset":9332,"parent_record_offset":9288,"ancestor_record_offset":8876,"inlinee":9802,"source_line":912,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4432,"record_offset":9256,"parent_record_offset":9100,"ancestor_record_offset":8876,"inlinee":7231,"source_line":931,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabRtConfig>
// 记录形式: compiled_procedure; instances=1
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4432,"record_offset":7108,"rva":32098576,"va":5400807696}
// 编译实例 RVAs: 0x1e9c910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2782956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400807696,"raw_hex":"e8ec762a00","size":5,"soft_fail":false,"target_va":5403591824,"va":5400808863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e90910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-49200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400807696,"raw_hex":"e8d03fffff","size":5,"soft_fail":false,"target_va":5400758544,"va":5400807739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ef2030
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t349808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400807696,"raw_hex":"e870560500","size":5,"soft_fail":false,"target_va":5401157680,"va":5400807867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400807696,"raw_hex":"e839e9ffff","size":5,"soft_fail":false,"target_va":5400802400,"va":5400808226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400807696,"raw_hex":"e8ebe8ffff","size":5,"soft_fail":false,"target_va":5400802624,"va":5400808528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":7328,"parent_record_offset":7108,"ancestor_record_offset":7108,"inlinee":121287,"source_line":175,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":7936,"parent_record_offset":7108,"ancestor_record_offset":7108,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":7976,"parent_record_offset":7108,"ancestor_record_offset":7108,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":7580,"parent_record_offset":7556,"ancestor_record_offset":7108,"inlinee":112761,"source_line":163,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4432,"record_offset":7812,"parent_record_offset":7328,"ancestor_record_offset":7108,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":7692,"parent_record_offset":7664,"ancestor_record_offset":7108,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":7444,"parent_record_offset":7328,"ancestor_record_offset":7108,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":7744,"parent_record_offset":7328,"ancestor_record_offset":7108,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4432,"record_offset":7408,"parent_record_offset":7384,"ancestor_record_offset":7108,"inlinee":30280,"source_line":32,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4432,"record_offset":7360,"parent_record_offset":7328,"ancestor_record_offset":7108,"inlinee":7227,"source_line":294,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_option
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4432,"record_offset":7512,"parent_record_offset":7480,"ancestor_record_offset":7108,"inlinee":121288,"source_line":1893,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4432,"record_offset":7384,"parent_record_offset":7360,"ancestor_record_offset":7108,"inlinee":9869,"source_line":1223,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4432,"record_offset":7556,"parent_record_offset":7512,"ancestor_record_offset":7108,"inlinee":9802,"source_line":912,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4432,"record_offset":7480,"parent_record_offset":7328,"ancestor_record_offset":7108,"inlinee":7231,"source_line":931,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4432,"record_offset":6192,"rva":32097344,"va":5400806464}
// 编译实例 RVAs: 0x1e9c440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2784281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400806464,"raw_hex":"e8197c2a00","size":5,"soft_fail":false,"target_va":5403591824,"va":5400807538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e90910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400806464,"raw_hex":"e8a044ffff","size":5,"soft_fail":false,"target_va":5400758544,"va":5400806507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ef0a80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t345466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400806464,"raw_hex":"e87a450500","size":5,"soft_fail":false,"target_va":5401152128,"va":5400806657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400806464,"raw_hex":"e813eeffff","size":5,"soft_fail":false,"target_va":5400802400,"va":5400806984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400806464,"raw_hex":"e8f3edffff","size":5,"soft_fail":false,"target_va":5400802624,"va":5400807240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4432,"record_offset":6424,"parent_record_offset":6192,"ancestor_record_offset":6192,"inlinee":121287,"source_line":175,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":7036,"parent_record_offset":6192,"ancestor_record_offset":6192,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":7076,"parent_record_offset":6192,"ancestor_record_offset":6192,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":6676,"parent_record_offset":6652,"ancestor_record_offset":6192,"inlinee":112761,"source_line":163,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4432,"record_offset":6912,"parent_record_offset":6424,"ancestor_record_offset":6192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":6792,"parent_record_offset":6764,"ancestor_record_offset":6192,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":6540,"parent_record_offset":6424,"ancestor_record_offset":6192,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":6844,"parent_record_offset":6424,"ancestor_record_offset":6192,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4432,"record_offset":6504,"parent_record_offset":6480,"ancestor_record_offset":6192,"inlinee":30280,"source_line":32,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4432,"record_offset":6456,"parent_record_offset":6424,"ancestor_record_offset":6192,"inlinee":7227,"source_line":294,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_option
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4432,"record_offset":6608,"parent_record_offset":6576,"ancestor_record_offset":6192,"inlinee":121288,"source_line":1893,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4432,"record_offset":6480,"parent_record_offset":6456,"ancestor_record_offset":6192,"inlinee":9869,"source_line":1223,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4432,"record_offset":6652,"parent_record_offset":6608,"ancestor_record_offset":6192,"inlinee":9802,"source_line":912,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4432,"record_offset":6576,"parent_record_offset":6424,"ancestor_record_offset":6192,"inlinee":7231,"source_line":931,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D9C7
// 首条位置: {"module":4432,"inlinee":121287}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":6940,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":112761,"source_line":163,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4432,"record_offset":6912,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":6872,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":6540,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":6844,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4432,"record_offset":6456,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":7227,"source_line":294,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4432,"record_offset":6576,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":7231,"source_line":931,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$96::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4432,"record_offset":8712,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":10271,"source_line":1338,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4432,"record_offset":6676,"parent_record_offset":6652,"ancestor_record_offset":6424,"inlinee":112761,"source_line":163,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4432,"record_offset":6792,"parent_record_offset":6764,"ancestor_record_offset":6424,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4432,"record_offset":6504,"parent_record_offset":6480,"ancestor_record_offset":6424,"inlinee":30280,"source_line":32,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_option
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4432,"record_offset":6608,"parent_record_offset":6576,"ancestor_record_offset":6424,"inlinee":121288,"source_line":1893,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4432,"record_offset":6480,"parent_record_offset":6456,"ancestor_record_offset":6424,"inlinee":9869,"source_line":1223,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4432,"record_offset":6652,"parent_record_offset":6608,"ancestor_record_offset":6424,"inlinee":9802,"source_line":912,"archive":"_pdb/A/modules/04432.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$96::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4432,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":8204,"inlinee":10271,"source_line":1338,"archive":"_pdb/A/modules/04432.jsonl.gz"}
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
// 完整结构及原始类型记录: inheritable.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::inheritable::Inheritable
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::Inheritable<asset::prefab::config::PrefabRenderConfig>", "constructor": "asset::inheritable::Inheritable", "arguments": ["asset::prefab::config::PrefabRenderConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<asset::inheritable::Inheritable<asset::prefab::config::PrefabRenderConfig> >", "constructor": "enum2$", "arguments": ["asset::inheritable::Inheritable<asset::prefab::config::PrefabRenderConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<asset::inheritable::Inheritable<asset::prefab::config::PrefabRenderConfig> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<asset::inheritable::Inheritable<asset::prefab::config::PrefabRenderConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: inheritable.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: inheritable.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::game_profile::SaveGameProfileCmd>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::game_profile::SaveGameProfileCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::game_profile::SaveGameProfileCmd>
// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > (*)(ref$<enum2$<serde::__private::de::content::Content> >)
// 候选参数类型: ["ref$<enum2$<serde::__private::de::content::Content> >"]

// ------------------------------------------------------------------
// 函数: asset::inheritable::Inheritable::as_concrete
// 信息 ID: c2955a301b6239a9cfb5fe15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 19 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=17808; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=17908; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=18356; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=18460; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=49444; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=50016; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=50280; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=50412; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::Inheritable::inherit_shallow
// 信息 ID: 10de21c284c369c738117cee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 24 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=4268; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=4364; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=4468; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=4540; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=10624; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=10712; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=10804; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=10864; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::Inheritable::into_concrete
// 信息 ID: b65b056f535dfd655de8a93e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::config::InheritableTextureOpts::into_concrete; module=4398, inline_record=392; 不是独立调用边。
// [PDB 内联宿主] asset::texture::config::InheritableTextureOpts::into_concrete; module=4398, inline_record=492; 不是独立调用边。
// [PDB 内联宿主] asset::texture::config::InheritableTextureOpts::into_concrete; module=4398, inline_record=604; 不是独立调用边。
// [PDB 内联宿主] asset::texture::config::InheritableTextureOpts::into_concrete; module=4398, inline_record=708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable
// 信息 ID: e841c733a2eb63e2c14f69e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 23 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4431, inline_record=34652; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4431, inline_record=36600; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4431, inline_record=41040; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4431, inline_record=43064; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4431, inline_record=51764; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4431, inline_record=52260; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4431, inline_record=52704; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4572, inline_record=8812; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
// 信息 ID: bdcbc1e9fa71217a9779c97b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >; module=4432, inline_record=6424; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabRtConfig>; module=4432, inline_record=7328; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,array$<u32,2> >; module=4432, inline_record=8204; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabAudioConfig>; module=4432, inline_record=9100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,array$<u32,2> >
// 信息 ID: d87f93629d2ce25f5da611eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::tag::Deserializer"]; 字段布局 = unknown。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::deserialize_inheritable_option -> ["ref_mut$<ron::de::tag::Deserializer>", "array$<u32,2>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor> @ 0x1e9a674 -> 0x1e9ce40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabAudioConfig>
// 信息 ID: 4a507db63c2184c8c1e89bd5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::tag::Deserializer"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::deserialize_inheritable_option -> ["ref_mut$<ron::de::tag::Deserializer>", "asset::prefab::config::PrefabAudioConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x1e97767 -> 0x1e9d280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabRtConfig>
// 信息 ID: 88e9fd5995a7e6ee67a01efd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::tag::Deserializer"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::deserialize_inheritable_option -> ["ref_mut$<ron::de::tag::Deserializer>", "asset::prefab::config::PrefabRtConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >
// 信息 ID: aa380f372cccdc2e9e91a2cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::tag::Deserializer"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["asset::prefab::config::MeshProcessorConfig"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::deserialize_inheritable_option -> ["ref_mut$<ron::de::tag::Deserializer>", "enum2$<asset::prefab::config::MeshProcessorConfig>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x1e95add -> 0x1e9c440 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$1::inherit_from<asset::prefab::config::AudioTriggerConfig>
// 信息 ID: ae30a407b281b059e66ccd28
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] asset::inheritable::impl$1::inherit_from -> ["asset::prefab::config::AudioTriggerConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > > @ 0x1e9be04 -> 0x1e9c100 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > > @ 0x1e9be49 -> 0x1e9c100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$1::inherit_from<asset::prefab::config::PrefabRenderConfig>
// 信息 ID: ddd0390dd300adf94e207a9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] asset::inheritable::impl$1::inherit_from -> ["asset::prefab::config::PrefabRenderConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x1ca3f28 -> 0x1e9b960 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x1ca4726 -> 0x1e9b960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >
// 信息 ID: 37e7129ae110c3d4789e0a80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::option::Option -> ["asset::prefab::config::PrefabAudioConfig"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<asset::prefab::config::PrefabAudioConfig>"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::impl$1::inherit_from -> ["enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x1ca3f49 -> 0x1e9bd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x1ca4746 -> 0x1e9bd80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > >
// 信息 ID: 46a6e1e1b9a061377dbd4d4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::option::Option -> ["asset::prefab::config::PrefabRtConfig"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<asset::prefab::config::PrefabRtConfig>"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::impl$1::inherit_from -> ["enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x1ca3f36 -> 0x1e9b730 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x1ca4736 -> 0x1e9b730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$3::inherit_from
// 信息 ID: b3b9235ccd77285c4ff78c40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=4432, inline_record=4572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$3::inherit_from<asset::prefab::config::PrefabRtConfig>
// 信息 ID: 3885699b095410817272d943
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] asset::inheritable::impl$3::inherit_from -> ["asset::prefab::config::PrefabRtConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > > @ 0x1e9b77a -> 0x2052e50 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$5::inherit_from::closure$0
// 信息 ID: 2bc771d100c4eab1b285c96d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset::inheritable::impl$5::inherit_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3976, inline_record=1532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$5::inherit_from::closure$1
// 信息 ID: 7eb1cee31a0f656116fd6c20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset::inheritable::impl$5::inherit_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3976, inline_record=2272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>
// 信息 ID: 1b4edf3fd5cc3bc62db48d4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] asset::inheritable::impl$5::inherit_from -> ["alloc::string::String", "asset::prefab::config::PrefabPartConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x18e786f -> 0x1ca3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x18e736e -> 0x1ca3df0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$7::clone
// 信息 ID: a069b9b5ee2fc443d1f47640
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 45 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=4300; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=4404; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=4508; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=4572; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=10648; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=10740; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=10832; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=10892; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
