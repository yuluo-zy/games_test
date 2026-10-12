// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/asset/src/inheritable.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5c19e731c2af936e8d8d2aacd637ee1e122731dfae6a9c2a530a2b3808f18ccf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: inheritable.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 555, 1553, 3167, 4091, 4141, 4583, 4589, 4590, 4591, 4592, 4605, 4742, 4753
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$7::clone
// 记录形式: inline_instance, inlinee_declaration; instances=86
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A66B
// 首条位置: {"module":3167,"inlinee":108139}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::impl$17::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":2052,"parent_record_offset":2004,"ancestor_record_offset":2004,"inlinee":124994,"source_line":71,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":3492,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":125000,"source_line":197,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_direct] core::array::impl$20::clone
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4091,"record_offset":2956,"parent_record_offset":2928,"ancestor_record_offset":2928,"inlinee":13478,"source_line":419,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":6828,"parent_record_offset":6800,"ancestor_record_offset":6800,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4091,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":2004,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4091,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":2004,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4091,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":2004,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4091,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":2004,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=28
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":2100,"parent_record_offset":2052,"ancestor_record_offset":2004,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":2204,"parent_record_offset":2172,"ancestor_record_offset":2004,"inlinee":71882,"source_line":108,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$26::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":2868,"parent_record_offset":2832,"ancestor_record_offset":2800,"inlinee":124996,"source_line":114,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$31::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":3408,"parent_record_offset":3368,"ancestor_record_offset":3332,"inlinee":124998,"source_line":175,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":3492,"parent_record_offset":3444,"ancestor_record_offset":3332,"inlinee":125000,"source_line":197,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4091,"record_offset":2956,"parent_record_offset":2928,"ancestor_record_offset":2800,"inlinee":13478,"source_line":419,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$22::clone
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4091,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":2800,"inlinee":13810,"source_line":442,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4091,"record_offset":2172,"parent_record_offset":2136,"ancestor_record_offset":2004,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4091,"record_offset":2284,"parent_record_offset":2052,"ancestor_record_offset":2004,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::Inheritable::inherit_shallow
// 记录形式: inline_instance, inlinee_declaration; instances=52
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A669
// 首条位置: {"module":3167,"inlinee":108137}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=19
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3167,"record_offset":6800,"parent_record_offset":6764,"ancestor_record_offset":6764,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4742,"record_offset":540,"parent_record_offset":512,"ancestor_record_offset":444,"inlinee":71882,"source_line":108,"archive":"_pdb/B/modules/04742.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4590,"record_offset":2592,"parent_record_offset":2564,"ancestor_record_offset":2532,"inlinee":13478,"source_line":419,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$22::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4590,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":2532,"inlinee":13810,"source_line":442,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":6828,"parent_record_offset":6800,"ancestor_record_offset":6764,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::Inheritable::as_concrete
// 记录形式: inline_instance, inlinee_declaration; instances=36
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7BF7
// 首条位置: {"module":1553,"inlinee":31735}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1553,"record_offset":56340,"parent_record_offset":56300,"ancestor_record_offset":56300,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1553,"record_offset":56368,"parent_record_offset":56300,"ancestor_record_offset":56300,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::Inheritable::into_concrete
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1EA3E
// 首条位置: {"module":4141,"inlinee":125502}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4141,"record_offset":444,"parent_record_offset":400,"ancestor_record_offset":400,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/04141.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4141,"record_offset":472,"parent_record_offset":400,"ancestor_record_offset":400,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/04141.jsonl.gz"}
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
// 首条位置: {"module":4590,"record_offset":2168,"rva":40410000,"va":5409119120}
// 编译实例 RVAs: 0x2689b90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-26271472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409119120,"raw_hex":"e810216ffe","size":5,"soft_fail":false,"target_va":5382847888,"va":5409119355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34038187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409119120,"raw_hex":"e8559ef8fd","size":5,"soft_fail":false,"target_va":5375081504,"va":5409119686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2422181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409119120,"raw_hex":"e8a5f52400","size":5,"soft_fail":false,"target_va":5411542064,"va":5409119878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2422045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409119120,"raw_hex":"e81df52400","size":5,"soft_fail":false,"target_va":5411541904,"va":5409119854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":2348,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":31735,"source_line":37,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":2736,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$10::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":2452,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":134094,"source_line":210,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4590,"record_offset":2944,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4590,"record_offset":3192,"parent_record_offset":3168,"ancestor_record_offset":2168,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4590,"record_offset":3168,"parent_record_offset":3112,"ancestor_record_offset":2168,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4590,"record_offset":3140,"parent_record_offset":3112,"ancestor_record_offset":2168,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4590,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":2168,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4590,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":2168,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":2496,"parent_record_offset":2452,"ancestor_record_offset":2168,"inlinee":108137,"source_line":30,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":2564,"parent_record_offset":2532,"ancestor_record_offset":2168,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":2780,"parent_record_offset":2736,"ancestor_record_offset":2168,"inlinee":125000,"source_line":197,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4590,"record_offset":2592,"parent_record_offset":2564,"ancestor_record_offset":2168,"inlinee":13478,"source_line":419,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$22::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4590,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":2168,"inlinee":13810,"source_line":442,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4590,"record_offset":2420,"parent_record_offset":2348,"ancestor_record_offset":2168,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4590,"record_offset":2392,"parent_record_offset":2348,"ancestor_record_offset":2168,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4590,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":2168,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04590.jsonl.gz"}
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
// 首条位置: {"module":4590,"record_offset":3260,"rva":40410832,"va":5409119952}
// 编译实例 RVAs: 0x2689ed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-26272322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409119952,"raw_hex":"e8be1d6ffe","size":5,"soft_fail":false,"target_va":5382847888,"va":5409120205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34038989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409119952,"raw_hex":"e8339bf8fd","size":5,"soft_fail":false,"target_va":5375081504,"va":5409120488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2421317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409119952,"raw_hex":"e845f22400","size":5,"soft_fail":false,"target_va":5411542064,"va":5409120742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2421181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409119952,"raw_hex":"e8bdf12400","size":5,"soft_fail":false,"target_va":5411541904,"va":5409120718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142879c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t2030791","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5409119952,"raw_hex":"e9c7fc1e00","size":5,"soft_fail":false,"target_va":5411150880,"va":5409120084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":3436,"parent_record_offset":3260,"ancestor_record_offset":3260,"inlinee":31735,"source_line":37,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":3540,"parent_record_offset":3260,"ancestor_record_offset":3260,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4590,"record_offset":4332,"parent_record_offset":3260,"ancestor_record_offset":3260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4590,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":3260,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4590,"record_offset":3964,"parent_record_offset":3936,"ancestor_record_offset":3260,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4590,"record_offset":4528,"parent_record_offset":4500,"ancestor_record_offset":3260,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4590,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":3260,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4590,"record_offset":3912,"parent_record_offset":3888,"ancestor_record_offset":3260,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":3628,"parent_record_offset":3580,"ancestor_record_offset":3260,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":3580,"parent_record_offset":3540,"ancestor_record_offset":3260,"inlinee":124994,"source_line":71,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":3744,"parent_record_offset":3712,"ancestor_record_offset":3260,"inlinee":71882,"source_line":108,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4590,"record_offset":3508,"parent_record_offset":3436,"ancestor_record_offset":3260,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4590,"record_offset":3480,"parent_record_offset":3436,"ancestor_record_offset":3260,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4590,"record_offset":3712,"parent_record_offset":3664,"ancestor_record_offset":3260,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4590,"record_offset":3816,"parent_record_offset":3580,"ancestor_record_offset":3260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04590.jsonl.gz"}
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
// 首条位置: {"module":4590,"record_offset":6384,"rva":40412928,"va":5409122048}
// 编译实例 RVAs: 0x268a700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26274565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409122048,"raw_hex":"e8fb146ffe","size":5,"soft_fail":false,"target_va":5382847888,"va":5409122448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34041047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409122048,"raw_hex":"e82993f8fd","size":5,"soft_fail":false,"target_va":5375081504,"va":5409122546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142689b90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409122048,"raw_hex":"e80ff4ffff","size":5,"soft_fail":false,"target_va":5409119120,"va":5409122172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2419221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409122048,"raw_hex":"e815ea2400","size":5,"soft_fail":false,"target_va":5411542064,"va":5409122838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2419085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409122048,"raw_hex":"e88de92400","size":5,"soft_fail":false,"target_va":5411541904,"va":5409122814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":6596,"parent_record_offset":6384,"ancestor_record_offset":6384,"inlinee":31735,"source_line":37,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$3::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":6700,"parent_record_offset":6384,"ancestor_record_offset":6384,"inlinee":134050,"source_line":99,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":6876,"parent_record_offset":6384,"ancestor_record_offset":6384,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4590,"record_offset":7192,"parent_record_offset":6384,"ancestor_record_offset":6384,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4590,"record_offset":7516,"parent_record_offset":7492,"ancestor_record_offset":6384,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4590,"record_offset":7492,"parent_record_offset":7436,"ancestor_record_offset":6384,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4590,"record_offset":7464,"parent_record_offset":7436,"ancestor_record_offset":6384,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4590,"record_offset":7436,"parent_record_offset":7412,"ancestor_record_offset":6384,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4590,"record_offset":7412,"parent_record_offset":7388,"ancestor_record_offset":6384,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$6::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":6384,"inlinee":108129,"source_line":152,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":6976,"parent_record_offset":6944,"ancestor_record_offset":6384,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$11::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":6384,"inlinee":134048,"source_line":223,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":6944,"parent_record_offset":6908,"ancestor_record_offset":6384,"inlinee":124998,"source_line":175,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":7016,"parent_record_offset":6976,"ancestor_record_offset":6384,"inlinee":125000,"source_line":197,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$8::inherit_from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":6732,"parent_record_offset":6700,"ancestor_record_offset":6384,"inlinee":134044,"source_line":182,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$9::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":6760,"parent_record_offset":6700,"ancestor_record_offset":6384,"inlinee":134046,"source_line":189,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4590,"record_offset":7080,"parent_record_offset":7052,"ancestor_record_offset":6384,"inlinee":13478,"source_line":419,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$22::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4590,"record_offset":7104,"parent_record_offset":7080,"ancestor_record_offset":6384,"inlinee":13810,"source_line":442,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4590,"record_offset":6668,"parent_record_offset":6596,"ancestor_record_offset":6384,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4590,"record_offset":6640,"parent_record_offset":6596,"ancestor_record_offset":6384,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4590,"record_offset":6908,"parent_record_offset":6876,"ancestor_record_offset":6384,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4590,"record_offset":7220,"parent_record_offset":7192,"ancestor_record_offset":6384,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04590.jsonl.gz"}
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
// 首条位置: {"module":4590,"record_offset":5216,"rva":40411872,"va":5409120992}
// 编译实例 RVAs: 0x268a2e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142879de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2030229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409120992,"raw_hex":"e895fa1e00","size":5,"soft_fail":false,"target_va":5411151328,"va":5409121094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2420093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409120992,"raw_hex":"e87ded2400","size":5,"soft_fail":false,"target_va":5411542064,"va":5409121966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2419957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409120992,"raw_hex":"e8f5ec2400","size":5,"soft_fail":false,"target_va":5411541904,"va":5409121942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":5420,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":31735,"source_line":37,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$3::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":5524,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":134050,"source_line":99,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":5848,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":5624,"parent_record_offset":5564,"ancestor_record_offset":5216,"inlinee":108137,"source_line":30,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":5972,"parent_record_offset":5912,"ancestor_record_offset":5216,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":5912,"parent_record_offset":5880,"ancestor_record_offset":5216,"inlinee":124996,"source_line":114,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$6::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":5564,"parent_record_offset":5524,"ancestor_record_offset":5216,"inlinee":134042,"source_line":136,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4590,"record_offset":6000,"parent_record_offset":5972,"ancestor_record_offset":5216,"inlinee":13478,"source_line":419,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$22::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4590,"record_offset":6024,"parent_record_offset":6000,"ancestor_record_offset":5216,"inlinee":13810,"source_line":442,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4590,"record_offset":5492,"parent_record_offset":5420,"ancestor_record_offset":5216,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4590,"record_offset":5464,"parent_record_offset":5420,"ancestor_record_offset":5216,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4590,"record_offset":5880,"parent_record_offset":5848,"ancestor_record_offset":5216,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$2::concrete_default
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A663
// 首条位置: {"module":3167,"inlinee":108131}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$3::inherit_from
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BA2
// 首条位置: {"module":4590,"inlinee":134050}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::impl$6::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":5564,"parent_record_offset":5524,"ancestor_record_offset":5524,"inlinee":134042,"source_line":136,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$8::inherit_from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":6732,"parent_record_offset":6700,"ancestor_record_offset":6700,"inlinee":134044,"source_line":182,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$9::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":6760,"parent_record_offset":6700,"ancestor_record_offset":6700,"inlinee":134046,"source_line":189,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":5624,"parent_record_offset":5564,"ancestor_record_offset":5524,"inlinee":108137,"source_line":30,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$6::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":6700,"inlinee":108129,"source_line":152,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$11::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":6700,"inlinee":134048,"source_line":223,"archive":"_pdb/B/modules/04590.jsonl.gz"}
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
// 首条位置: {"module":4091,"record_offset":252,"rva":37899664,"va":5406608784}
// 编译实例 RVAs: 0x2424d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-23761172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406608784,"raw_hex":"e8ec6e95fe","size":5,"soft_fail":false,"target_va":5382847888,"va":5406609055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31530114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406608784,"raw_hex":"e87ee31efe","size":5,"soft_fail":false,"target_va":5375081504,"va":5406611613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088a680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28947557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406608784,"raw_hex":"e89b4b46fe","size":5,"soft_fail":false,"target_va":5377664640,"va":5406612192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142422a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406608784,"raw_hex":"e85ddbffff","size":5,"soft_fail":false,"target_va":5406599696,"va":5406609070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142689ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2511066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406608784,"raw_hex":"e8da502600","size":5,"soft_fail":false,"target_va":5409119952,"va":5406608881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14268a2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2512094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406608784,"raw_hex":"e8de542600","size":5,"soft_fail":false,"target_va":5409120992,"va":5406608893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14268a700
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2513128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406608784,"raw_hex":"e8e8582600","size":5,"soft_fail":false,"target_va":5409122048,"va":5406608915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$asset..prefab..config..PrefabRenderConfig$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406608784,"raw_hex":"e815ecffff","size":5,"soft_fail":false,"target_va":5406606736,"va":5406611830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::prefab::config::impl$2::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":6228,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":125002,"source_line":44,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$3::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":6256,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":124989,"source_line":55,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4091,"record_offset":6328,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::and_modify
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4091,"record_offset":828,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":24493,"source_line":2900,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4091,"record_offset":1816,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":7951,"source_line":2822,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4091,"record_offset":1760,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":7716,"source_line":872,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4091,"record_offset":5148,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":13598,"source_line":473,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$44::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4091,"record_offset":476,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5192,"source_line":2340,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4091,"record_offset":1008,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$64::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4091,"record_offset":5524,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14696,"source_line":2586,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4091,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":252,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4091,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":252,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4091,"record_offset":5032,"parent_record_offset":5004,"ancestor_record_offset":252,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4091,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":252,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4091,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":252,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":932,"parent_record_offset":888,"ancestor_record_offset":252,"inlinee":108137,"source_line":30,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$5::inherit_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":856,"parent_record_offset":828,"ancestor_record_offset":252,"inlinee":124987,"source_line":131,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$5::inherit_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":1864,"parent_record_offset":1816,"ancestor_record_offset":252,"inlinee":124990,"source_line":132,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=16
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":964,"parent_record_offset":932,"ancestor_record_offset":252,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$14::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":1904,"parent_record_offset":1864,"ancestor_record_offset":252,"inlinee":124992,"source_line":27,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":2052,"parent_record_offset":2004,"ancestor_record_offset":252,"inlinee":124994,"source_line":71,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":2204,"parent_record_offset":2172,"ancestor_record_offset":252,"inlinee":71882,"source_line":108,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":2868,"parent_record_offset":2832,"ancestor_record_offset":252,"inlinee":124996,"source_line":114,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":3408,"parent_record_offset":3368,"ancestor_record_offset":252,"inlinee":124998,"source_line":175,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":3492,"parent_record_offset":3444,"ancestor_record_offset":252,"inlinee":125000,"source_line":197,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$3::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":888,"parent_record_offset":856,"ancestor_record_offset":252,"inlinee":124989,"source_line":55,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4091,"record_offset":2956,"parent_record_offset":2928,"ancestor_record_offset":252,"inlinee":13478,"source_line":419,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$22::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4091,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":252,"inlinee":13810,"source_line":442,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4091,"record_offset":680,"parent_record_offset":652,"ancestor_record_offset":252,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4091,"record_offset":3980,"parent_record_offset":3952,"ancestor_record_offset":252,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4091,"record_offset":740,"parent_record_offset":708,"ancestor_record_offset":252,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4091,"record_offset":4008,"parent_record_offset":3980,"ancestor_record_offset":252,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4091,"record_offset":4180,"parent_record_offset":4048,"ancestor_record_offset":252,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4091,"record_offset":4612,"parent_record_offset":4572,"ancestor_record_offset":252,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":4091,"record_offset":1284,"parent_record_offset":1252,"ancestor_record_offset":252,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":4091,"record_offset":1252,"parent_record_offset":1220,"ancestor_record_offset":252,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4091,"record_offset":2172,"parent_record_offset":2136,"ancestor_record_offset":252,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4091,"record_offset":776,"parent_record_offset":628,"ancestor_record_offset":252,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4091,"record_offset":2284,"parent_record_offset":2052,"ancestor_record_offset":252,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4091,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":252,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4091,"record_offset":4788,"parent_record_offset":4764,"ancestor_record_offset":252,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4091,"record_offset":4812,"parent_record_offset":4788,"ancestor_record_offset":252,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":4091,"record_offset":1180,"parent_record_offset":1140,"ancestor_record_offset":252,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":4091,"record_offset":1220,"parent_record_offset":1180,"ancestor_record_offset":252,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":4091,"record_offset":1328,"parent_record_offset":1140,"ancestor_record_offset":252,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":4091,"record_offset":1140,"parent_record_offset":1092,"ancestor_record_offset":252,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":4091,"record_offset":3952,"parent_record_offset":3864,"ancestor_record_offset":252,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":4091,"record_offset":652,"parent_record_offset":628,"ancestor_record_offset":252,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":4091,"record_offset":708,"parent_record_offset":652,"ancestor_record_offset":252,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":4091,"record_offset":1420,"parent_record_offset":1092,"ancestor_record_offset":252,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":4091,"record_offset":4652,"parent_record_offset":3776,"ancestor_record_offset":252,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":4091,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":252,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":4091,"record_offset":5204,"parent_record_offset":5176,"ancestor_record_offset":252,"inlinee":13574,"source_line":799,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":4091,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":252,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":4091,"record_offset":5580,"parent_record_offset":5552,"ancestor_record_offset":252,"inlinee":11990,"source_line":3219,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":1692,"parent_record_offset":1668,"ancestor_record_offset":252,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":1668,"parent_record_offset":1032,"ancestor_record_offset":252,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":4704,"parent_record_offset":4680,"ancestor_record_offset":252,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":1520,"parent_record_offset":1092,"ancestor_record_offset":252,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":4764,"parent_record_offset":3776,"ancestor_record_offset":252,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":4516,"parent_record_offset":3864,"ancestor_record_offset":252,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":628,"parent_record_offset":572,"ancestor_record_offset":252,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":1092,"parent_record_offset":1064,"ancestor_record_offset":252,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":3776,"parent_record_offset":3744,"ancestor_record_offset":252,"inlinee":6287,"source_line":1062,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":252,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":4680,"parent_record_offset":3776,"ancestor_record_offset":252,"inlinee":6232,"source_line":2195,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":600,"parent_record_offset":572,"ancestor_record_offset":252,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":3864,"parent_record_offset":3824,"ancestor_record_offset":252,"inlinee":6231,"source_line":1805,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":4048,"parent_record_offset":3864,"ancestor_record_offset":252,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":4244,"parent_record_offset":3864,"ancestor_record_offset":252,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":4280,"parent_record_offset":4244,"ancestor_record_offset":252,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":252,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":3824,"parent_record_offset":3776,"ancestor_record_offset":252,"inlinee":6230,"source_line":1762,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":3924,"parent_record_offset":3864,"ancestor_record_offset":252,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":4572,"parent_record_offset":4548,"ancestor_record_offset":252,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":4548,"parent_record_offset":3824,"ancestor_record_offset":252,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4091,"record_offset":1064,"parent_record_offset":1032,"ancestor_record_offset":252,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":4091,"record_offset":3744,"parent_record_offset":3712,"ancestor_record_offset":252,"inlinee":7725,"source_line":512,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4091,"record_offset":500,"parent_record_offset":476,"ancestor_record_offset":252,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4091,"record_offset":5176,"parent_record_offset":5148,"ancestor_record_offset":252,"inlinee":13573,"source_line":570,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4091,"record_offset":3712,"parent_record_offset":1816,"ancestor_record_offset":252,"inlinee":7723,"source_line":3192,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$50::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4091,"record_offset":5552,"parent_record_offset":5524,"ancestor_record_offset":252,"inlinee":14698,"source_line":2428,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4091,"record_offset":1784,"parent_record_offset":1760,"ancestor_record_offset":252,"inlinee":17354,"source_line":3285,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$5::inherit_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E83B
// 首条位置: {"module":4091,"inlinee":124987}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::impl$3::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":888,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":124989,"source_line":55,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":932,"parent_record_offset":888,"ancestor_record_offset":856,"inlinee":108137,"source_line":30,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":964,"parent_record_offset":932,"ancestor_record_offset":856,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$5::inherit_from::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E83E
// 首条位置: {"module":4091,"inlinee":124990}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::impl$14::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":1904,"parent_record_offset":1864,"ancestor_record_offset":1864,"inlinee":124992,"source_line":27,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4091,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":1864,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4091,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":1864,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4091,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":1864,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4091,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":1864,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=15
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":1976,"parent_record_offset":1904,"ancestor_record_offset":1864,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":2052,"parent_record_offset":2004,"ancestor_record_offset":1864,"inlinee":124994,"source_line":71,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":2204,"parent_record_offset":2172,"ancestor_record_offset":1864,"inlinee":71882,"source_line":108,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":2868,"parent_record_offset":2832,"ancestor_record_offset":1864,"inlinee":124996,"source_line":114,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":3408,"parent_record_offset":3368,"ancestor_record_offset":1864,"inlinee":124998,"source_line":175,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":3492,"parent_record_offset":3444,"ancestor_record_offset":1864,"inlinee":125000,"source_line":197,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4091,"record_offset":2956,"parent_record_offset":2928,"ancestor_record_offset":1864,"inlinee":13478,"source_line":419,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$22::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4091,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":1864,"inlinee":13810,"source_line":442,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4091,"record_offset":2172,"parent_record_offset":2136,"ancestor_record_offset":1864,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4091,"record_offset":2284,"parent_record_offset":2052,"ancestor_record_offset":1864,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$6::concrete_default
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A661
// 首条位置: {"module":3167,"inlinee":108129}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$2::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3167,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":3152,"inlinee":108131,"source_line":90,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$11::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":6784,"inlinee":134048,"source_line":223,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable
// 记录形式: inline_instance, inlinee_declaration; instances=57
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BB7
// 首条位置: {"module":4589,"inlinee":134071}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4589,"record_offset":54024,"parent_record_offset":54000,"ancestor_record_offset":54000,"inlinee":125015,"source_line":71,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4589,"record_offset":33276,"parent_record_offset":33252,"ancestor_record_offset":33252,"inlinee":108179,"source_line":4,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":33332,"parent_record_offset":33252,"ancestor_record_offset":33252,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":52136,"parent_record_offset":52112,"ancestor_record_offset":52112,"inlinee":6537,"source_line":72,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":44192,"parent_record_offset":44156,"ancestor_record_offset":44156,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$99::deserialize
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":18276,"parent_record_offset":18212,"ancestor_record_offset":18212,"inlinee":6681,"source_line":1338,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":44952,"parent_record_offset":44928,"ancestor_record_offset":44156,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":44976,"parent_record_offset":44952,"ancestor_record_offset":44156,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":44928,"parent_record_offset":44752,"ancestor_record_offset":44156,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":44692,"parent_record_offset":44664,"ancestor_record_offset":44156,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":44752,"parent_record_offset":44720,"ancestor_record_offset":44156,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":44720,"parent_record_offset":44692,"ancestor_record_offset":44156,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":44788,"parent_record_offset":44752,"ancestor_record_offset":44156,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":44624,"parent_record_offset":44588,"ancestor_record_offset":44156,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":44588,"parent_record_offset":44552,"ancestor_record_offset":44156,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":44516,"parent_record_offset":44480,"ancestor_record_offset":44156,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":44552,"parent_record_offset":44516,"ancestor_record_offset":44156,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":44480,"parent_record_offset":44444,"ancestor_record_offset":44156,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":4589,"record_offset":44444,"parent_record_offset":44408,"ancestor_record_offset":44156,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4589,"record_offset":44664,"parent_record_offset":44624,"ancestor_record_offset":44156,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4589,"record_offset":44812,"parent_record_offset":44788,"ancestor_record_offset":44156,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4589,"record_offset":44836,"parent_record_offset":44812,"ancestor_record_offset":44156,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4589,"record_offset":45052,"parent_record_offset":45028,"ancestor_record_offset":44156,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4589,"record_offset":44884,"parent_record_offset":44860,"ancestor_record_offset":44156,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":44860,"parent_record_offset":44836,"ancestor_record_offset":44156,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4589,"record_offset":45028,"parent_record_offset":44624,"ancestor_record_offset":44156,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":44340,"parent_record_offset":44300,"ancestor_record_offset":44156,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_bool
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":52184,"parent_record_offset":52160,"ancestor_record_offset":52112,"inlinee":6888,"source_line":345,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":44300,"parent_record_offset":44264,"ancestor_record_offset":44156,"inlinee":5163,"source_line":445,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":44264,"parent_record_offset":44228,"ancestor_record_offset":44156,"inlinee":5162,"source_line":455,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_bool
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":52160,"parent_record_offset":52136,"ancestor_record_offset":52112,"inlinee":6908,"source_line":39,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":33300,"parent_record_offset":33276,"ancestor_record_offset":33252,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_string
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":44228,"parent_record_offset":44192,"ancestor_record_offset":44156,"inlinee":5160,"source_line":139,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":54048,"parent_record_offset":54024,"ancestor_record_offset":54000,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":44372,"parent_record_offset":44300,"ancestor_record_offset":44156,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$6::visit_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":44408,"parent_record_offset":44372,"ancestor_record_offset":44156,"inlinee":6423,"source_line":603,"archive":"_pdb/B/modules/04589.jsonl.gz"}
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
// 首条位置: {"module":4590,"record_offset":8500,"rva":40414944,"va":5409124064}
// 编译实例 RVAs: 0x268aee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2718051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409124064,"raw_hex":"e863792900","size":5,"soft_fail":false,"target_va":5411842768,"va":5409124712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14267f570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409124064,"raw_hex":"e85e46ffff","size":5,"soft_fail":false,"target_va":5409076592,"va":5409124109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1426920f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409124064,"raw_hex":"e879710000","size":5,"soft_fail":false,"target_va":5409153264,"va":5409124210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409124064,"raw_hex":"e8fbe7ffff","size":5,"soft_fail":false,"target_va":5409118416,"va":5409124560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409124064,"raw_hex":"e8cfe7ffff","size":5,"soft_fail":false,"target_va":5409118624,"va":5409124812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":8696,"parent_record_offset":8500,"ancestor_record_offset":8500,"inlinee":134098,"source_line":175,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":9312,"parent_record_offset":8500,"ancestor_record_offset":8500,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":9348,"parent_record_offset":8500,"ancestor_record_offset":8500,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4590,"record_offset":9188,"parent_record_offset":8696,"ancestor_record_offset":8500,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":9104,"parent_record_offset":9076,"ancestor_record_offset":8500,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":8844,"parent_record_offset":8696,"ancestor_record_offset":8500,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4590,"record_offset":8808,"parent_record_offset":8784,"ancestor_record_offset":8500,"inlinee":38808,"source_line":32,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4590,"record_offset":8760,"parent_record_offset":8696,"ancestor_record_offset":8500,"inlinee":6541,"source_line":294,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_option
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4590,"record_offset":8912,"parent_record_offset":8880,"ancestor_record_offset":8500,"inlinee":134040,"source_line":1893,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4590,"record_offset":8784,"parent_record_offset":8760,"ancestor_record_offset":8500,"inlinee":6543,"source_line":1223,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4590,"record_offset":8964,"parent_record_offset":8912,"ancestor_record_offset":8500,"inlinee":11509,"source_line":912,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4590,"record_offset":8880,"parent_record_offset":8696,"ancestor_record_offset":8500,"inlinee":6990,"source_line":931,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$96::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4590,"record_offset":8988,"parent_record_offset":8964,"ancestor_record_offset":8500,"inlinee":13361,"source_line":1338,"archive":"_pdb/B/modules/04590.jsonl.gz"}
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
// 首条位置: {"module":4590,"record_offset":9380,"rva":40415840,"va":5409124960}
// 编译实例 RVAs: 0x268b260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2716754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409124960,"raw_hex":"e852742900","size":5,"soft_fail":false,"target_va":5411842768,"va":5409126009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14267f570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-48418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409124960,"raw_hex":"e8de42ffff","size":5,"soft_fail":false,"target_va":5409076592,"va":5409125005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1426928c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409124960,"raw_hex":"e8a3750000","size":5,"soft_fail":false,"target_va":5409155264,"va":5409125144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409124960,"raw_hex":"e86ee4ffff","size":5,"soft_fail":false,"target_va":5409118416,"va":5409125469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-7164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409124960,"raw_hex":"e804e4ffff","size":5,"soft_fail":false,"target_va":5409118624,"va":5409125783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":9604,"parent_record_offset":9380,"ancestor_record_offset":9380,"inlinee":134098,"source_line":175,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":10208,"parent_record_offset":9380,"ancestor_record_offset":9380,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":10252,"parent_record_offset":9380,"ancestor_record_offset":9380,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":9876,"parent_record_offset":9852,"ancestor_record_offset":9380,"inlinee":125015,"source_line":114,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4590,"record_offset":10084,"parent_record_offset":9604,"ancestor_record_offset":9380,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":9992,"parent_record_offset":9964,"ancestor_record_offset":9380,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":9740,"parent_record_offset":9604,"ancestor_record_offset":9380,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4590,"record_offset":9704,"parent_record_offset":9680,"ancestor_record_offset":9380,"inlinee":38808,"source_line":32,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4590,"record_offset":9656,"parent_record_offset":9604,"ancestor_record_offset":9380,"inlinee":6541,"source_line":294,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_option
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4590,"record_offset":9808,"parent_record_offset":9776,"ancestor_record_offset":9380,"inlinee":134040,"source_line":1893,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4590,"record_offset":9680,"parent_record_offset":9656,"ancestor_record_offset":9380,"inlinee":6543,"source_line":1223,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4590,"record_offset":9852,"parent_record_offset":9808,"ancestor_record_offset":9380,"inlinee":11509,"source_line":912,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4590,"record_offset":9776,"parent_record_offset":9604,"ancestor_record_offset":9380,"inlinee":6990,"source_line":931,"archive":"_pdb/B/modules/04590.jsonl.gz"}
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
// 首条位置: {"module":4590,"record_offset":10284,"rva":40417136,"va":5409126256}
// 编译实例 RVAs: 0x268b770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2715256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409126256,"raw_hex":"e8786e2900","size":5,"soft_fail":false,"target_va":5411842768,"va":5409127507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14267f570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-49712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409126256,"raw_hex":"e8d03dffff","size":5,"soft_fail":false,"target_va":5409076592,"va":5409126299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1426930c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409126256,"raw_hex":"e89c780000","size":5,"soft_fail":false,"target_va":5409157312,"va":5409126431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409126256,"raw_hex":"e8cfddffff","size":5,"soft_fail":false,"target_va":5409118416,"va":5409127164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-8287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409126256,"raw_hex":"e8a1dfffff","size":5,"soft_fail":false,"target_va":5409118624,"va":5409126906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":10504,"parent_record_offset":10284,"ancestor_record_offset":10284,"inlinee":134098,"source_line":175,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":11068,"parent_record_offset":10284,"ancestor_record_offset":10284,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":11112,"parent_record_offset":10284,"ancestor_record_offset":10284,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":10752,"parent_record_offset":10728,"ancestor_record_offset":10284,"inlinee":125015,"source_line":114,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4590,"record_offset":10944,"parent_record_offset":10504,"ancestor_record_offset":10284,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":10860,"parent_record_offset":10836,"ancestor_record_offset":10284,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":10624,"parent_record_offset":10504,"ancestor_record_offset":10284,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4590,"record_offset":10588,"parent_record_offset":10564,"ancestor_record_offset":10284,"inlinee":38808,"source_line":32,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4590,"record_offset":10540,"parent_record_offset":10504,"ancestor_record_offset":10284,"inlinee":6541,"source_line":294,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_option
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4590,"record_offset":10688,"parent_record_offset":10660,"ancestor_record_offset":10284,"inlinee":134040,"source_line":1893,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4590,"record_offset":10564,"parent_record_offset":10540,"ancestor_record_offset":10284,"inlinee":6543,"source_line":1223,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4590,"record_offset":10728,"parent_record_offset":10688,"ancestor_record_offset":10284,"inlinee":11509,"source_line":912,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4590,"record_offset":10660,"parent_record_offset":10504,"ancestor_record_offset":10284,"inlinee":6990,"source_line":931,"archive":"_pdb/B/modules/04590.jsonl.gz"}
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
// 首条位置: {"module":4590,"record_offset":7596,"rva":40413792,"va":5409122912}
// 编译实例 RVAs: 0x268aa60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2718859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409122912,"raw_hex":"e88b7c2900","size":5,"soft_fail":false,"target_va":5411842768,"va":5409123904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14267f570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-46368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409122912,"raw_hex":"e8e04affff","size":5,"soft_fail":false,"target_va":5409076592,"va":5409122955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142691cd0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409122912,"raw_hex":"e8b5710000","size":5,"soft_fail":false,"target_va":5409152208,"va":5409123094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409122912,"raw_hex":"e881ecffff","size":5,"soft_fail":false,"target_va":5409118416,"va":5409123402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409122912,"raw_hex":"e851ecffff","size":5,"soft_fail":false,"target_va":5409118624,"va":5409123658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":7828,"parent_record_offset":7596,"ancestor_record_offset":7596,"inlinee":134098,"source_line":175,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":8428,"parent_record_offset":7596,"ancestor_record_offset":7596,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":8468,"parent_record_offset":7596,"ancestor_record_offset":7596,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":7596,"inlinee":125015,"source_line":114,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4590,"record_offset":8304,"parent_record_offset":7828,"ancestor_record_offset":7596,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":8212,"parent_record_offset":8184,"ancestor_record_offset":7596,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":7960,"parent_record_offset":7828,"ancestor_record_offset":7596,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4590,"record_offset":7924,"parent_record_offset":7900,"ancestor_record_offset":7596,"inlinee":38808,"source_line":32,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4590,"record_offset":7876,"parent_record_offset":7828,"ancestor_record_offset":7596,"inlinee":6541,"source_line":294,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_option
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4590,"record_offset":8028,"parent_record_offset":7996,"ancestor_record_offset":7596,"inlinee":134040,"source_line":1893,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4590,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":7596,"inlinee":6543,"source_line":1223,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4590,"record_offset":8072,"parent_record_offset":8028,"ancestor_record_offset":7596,"inlinee":11509,"source_line":912,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4590,"record_offset":7996,"parent_record_offset":7828,"ancestor_record_offset":7596,"inlinee":6990,"source_line":931,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BD2
// 首条位置: {"module":4590,"inlinee":134098}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":8332,"parent_record_offset":7828,"ancestor_record_offset":7828,"inlinee":125015,"source_line":114,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4590,"record_offset":8304,"parent_record_offset":7828,"ancestor_record_offset":7828,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":8264,"parent_record_offset":7828,"ancestor_record_offset":7828,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":7960,"parent_record_offset":7828,"ancestor_record_offset":7828,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4590,"record_offset":7876,"parent_record_offset":7828,"ancestor_record_offset":7828,"inlinee":6541,"source_line":294,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4590,"record_offset":7996,"parent_record_offset":7828,"ancestor_record_offset":7828,"inlinee":6990,"source_line":931,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$96::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4590,"record_offset":9216,"parent_record_offset":8696,"ancestor_record_offset":8696,"inlinee":13361,"source_line":1338,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":7828,"inlinee":125015,"source_line":114,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4590,"record_offset":8212,"parent_record_offset":8184,"ancestor_record_offset":7828,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4590,"record_offset":7924,"parent_record_offset":7900,"ancestor_record_offset":7828,"inlinee":38808,"source_line":32,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_option
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4590,"record_offset":8028,"parent_record_offset":7996,"ancestor_record_offset":7828,"inlinee":134040,"source_line":1893,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4590,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":7828,"inlinee":6543,"source_line":1223,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4590,"record_offset":8072,"parent_record_offset":8028,"ancestor_record_offset":7828,"inlinee":11509,"source_line":912,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$96::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4590,"record_offset":8988,"parent_record_offset":8964,"ancestor_record_offset":8696,"inlinee":13361,"source_line":1338,"archive":"_pdb/B/modules/04590.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > (*)(ref$<enum2$<serde::__private::de::content::Content> >)
// 候选参数类型: ["ref$<enum2$<serde::__private::de::content::Content> >"]

// ------------------------------------------------------------------
// 函数: asset::inheritable::Inheritable::as_concrete
// 信息 ID: ab711644f745ea409aa226d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 21 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs_impl; module=1553, inline_record=56300; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs_impl; module=1553, inline_record=56400; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs_impl; module=1553, inline_record=56844; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs_impl; module=1553, inline_record=56948; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs_impl; module=1553, inline_record=89160; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs_impl; module=1553, inline_record=89700; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs_impl; module=1553, inline_record=89996; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs_impl; module=1553, inline_record=90132; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::Inheritable::inherit_shallow
// 信息 ID: 0675bc32e455d307b2d42070
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 28 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=6764; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=6864; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=6936; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=7008; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=16640; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=16728; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=16788; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=16848; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::Inheritable::into_concrete
// 信息 ID: 0d74045a27ad91a4813c973d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::config::InheritableTextureOpts::into_concrete; module=4141, inline_record=400; 不是独立调用边。
// [PDB 内联宿主] asset::texture::config::InheritableTextureOpts::into_concrete; module=4141, inline_record=504; 不是独立调用边。
// [PDB 内联宿主] asset::texture::config::InheritableTextureOpts::into_concrete; module=4141, inline_record=608; 不是独立调用边。
// [PDB 内联宿主] asset::texture::config::InheritableTextureOpts::into_concrete; module=4141, inline_record=712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable
// 信息 ID: e04d80a9027a78758ec81159
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e55b91db0419e2d4eecc727d: serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > (*)(ref$<enum2$<serde::__private::de::content::Content> >)
// 引用方: 原程序直接转移 0 条；内联宿主 27 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=33252; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=33712; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=34120; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=44156; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=46044; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=52112; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=54000; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=14760; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
// 信息 ID: 4bb82e47c6382910af3f3afe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >; module=4590, inline_record=7828; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,array$<u32,2> >; module=4590, inline_record=8696; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabAudioConfig>; module=4590, inline_record=9604; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabRtConfig>; module=4590, inline_record=10504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,array$<u32,2> >
// 信息 ID: 8d424bf3cad416de22a4f48f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::tag::Deserializer"]; 字段布局 = unknown。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::deserialize_inheritable_option -> ["ref_mut$<ron::de::tag::Deserializer>", "array$<u32,2>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor> @ 0x2682a5c -> 0x268aee0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabAudioConfig>
// 信息 ID: d5469bf86a1cedf1f10c237a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::tag::Deserializer"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::deserialize_inheritable_option -> ["ref_mut$<ron::de::tag::Deserializer>", "asset::prefab::config::PrefabAudioConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x2687dbb -> 0x268b260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabRtConfig>
// 信息 ID: 27ff6f69687fff03ac05f115
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::tag::Deserializer"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::deserialize_inheritable_option -> ["ref_mut$<ron::de::tag::Deserializer>", "asset::prefab::config::PrefabRtConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >
// 信息 ID: dcd09f1e1d5d7587b03823b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::tag::Deserializer"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["asset::prefab::config::MeshProcessorConfig"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::deserialize_inheritable_option -> ["ref_mut$<ron::de::tag::Deserializer>", "enum2$<asset::prefab::config::MeshProcessorConfig>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$1::inherit_from<asset::prefab::config::AudioTriggerConfig>
// 信息 ID: 8b8e49572e7314219f8a6a1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] asset::inheritable::impl$1::inherit_from -> ["asset::prefab::config::AudioTriggerConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > > @ 0x268a77c -> 0x2689b90 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > > @ 0x268a7c7 -> 0x2689b90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$1::inherit_from<asset::prefab::config::PrefabRenderConfig>
// 信息 ID: 5708716b9a23b46b2d14560f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] asset::inheritable::impl$1::inherit_from -> ["asset::prefab::config::PrefabRenderConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x2424df1 -> 0x2689ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x242584e -> 0x2689ed0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >
// 信息 ID: 539bb47d75ddef687d9c2e12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::option::Option -> ["asset::prefab::config::PrefabAudioConfig"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<asset::prefab::config::PrefabAudioConfig>"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::impl$1::inherit_from -> ["enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x2424e13 -> 0x268a700 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x242586b -> 0x268a700 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > >
// 信息 ID: 2ecfcbebdff1942ece84b92d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::option::Option -> ["asset::prefab::config::PrefabRtConfig"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<asset::prefab::config::PrefabRtConfig>"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::impl$1::inherit_from -> ["enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x2424dfd -> 0x268a2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x242585b -> 0x268a2e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$2::concrete_default
// 信息 ID: 80604342833931065398cd7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree; module=3167, inline_record=3176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$3::inherit_from
// 信息 ID: 0f0b274f8fd6a1ffb7413461
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > >; module=4590, inline_record=5524; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=4590, inline_record=6700; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$5::inherit_from::closure$0
// 信息 ID: 9494498b9853a973d59f98f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset::inheritable::impl$5::inherit_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=4091, inline_record=856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$5::inherit_from::closure$1
// 信息 ID: aa21aa6d3e4fd2a80e87d58b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset::inheritable::impl$5::inherit_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=4091, inline_record=1864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>
// 信息 ID: f50cb1120f0528d899ac75d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] asset::inheritable::impl$5::inherit_from -> ["alloc::string::String", "asset::prefab::config::PrefabPartConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x200ad0d -> 0x2424d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x200a81d -> 0x2424d90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$6::concrete_default
// 信息 ID: 7e8cd0c3664ec0a7bf261261
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree; module=3167, inline_record=3152; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=4590, inline_record=6784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$7::clone
// 信息 ID: acb229de609a646c3c35da47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 51 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=6800; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=6904; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=6976; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=7040; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=16664; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=16756; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=16816; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=16876; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
