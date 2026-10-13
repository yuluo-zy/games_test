// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/water/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e91c6dada2baf4702813ac394683b585cff9b54ad36f04d8e10682907f95cebf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 302, 1010, 2433, 2436, 2440, 3638, 3644, 3648, 3650, 4679, 4681
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use zeno; // namespace candidate
//   package identities: [{"name":"zeno","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f","version":"0.2.2"},{"name":"zeno","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2436,"record_offset":1452,"rva":29027744,"va":5397736864}
// 编译实例 RVAs: 0x1baeda0, 0x2263020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t13804928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397736864,"raw_hex":"e880a5d200","size":5,"soft_fail":false,"target_va":5411542296,"va":5397737363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":2436,"record_offset":1728,"parent_record_offset":1452,"ancestor_record_offset":1452,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/02436.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2436,"record_offset":1856,"parent_record_offset":1452,"ancestor_record_offset":1452,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/02436.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2436,"record_offset":1636,"parent_record_offset":1452,"ancestor_record_offset":1452,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/02436.jsonl.gz"}
//   [pdb_inline_direct] core::str::pattern::impl$36::matching
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2436,"record_offset":2008,"parent_record_offset":1452,"ancestor_record_offset":1452,"inlinee":5012,"source_line":1736,"archive":"_pdb/B/modules/02436.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2436,"record_offset":1812,"parent_record_offset":1784,"ancestor_record_offset":1452,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/02436.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2436,"record_offset":1944,"parent_record_offset":1884,"ancestor_record_offset":1452,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/02436.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2436,"record_offset":1884,"parent_record_offset":1856,"ancestor_record_offset":1452,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/02436.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2436,"record_offset":1784,"parent_record_offset":1756,"ancestor_record_offset":1452,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/02436.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2436,"record_offset":1756,"parent_record_offset":1728,"ancestor_record_offset":1452,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/02436.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2436,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":1452,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/02436.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2436,"record_offset":1688,"parent_record_offset":1636,"ancestor_record_offset":1452,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/02436.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zeno::raster::Rasterizer::curve_to<zeno::raster::AdaptiveStorage>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4679,"record_offset":57728,"rva":42119216,"va":5410828336}
// 编译实例 RVAs: 0x282b030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428290f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410828336,"raw_hex":"e806e0ffff","size":5,"soft_fail":false,"target_va":5410820336,"va":5410828517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t713504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410828336,"raw_hex":"e820e30a00","size":5,"soft_fail":false,"target_va":5411542296,"va":5410828787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::raster::split_cubic
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-17372231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410828336,"raw_hex":"e8b9ebf6fe","size":5,"soft_fail":false,"target_va":5393456528,"va":5410828754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::impl$16::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4679,"record_offset":58028,"parent_record_offset":57728,"ancestor_record_offset":57728,"inlinee":10912,"source_line":374,"archive":"_pdb/B/modules/04679.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$2::abs
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":4679,"record_offset":57916,"parent_record_offset":57728,"ancestor_record_offset":57728,"inlinee":14836,"source_line":3449,"archive":"_pdb/B/modules/04679.jsonl.gz"}
//   [pdb_inline_direct] zeno::raster::trunc
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/raster.rs
//     evidence={"module":4679,"record_offset":57888,"parent_record_offset":57728,"ancestor_record_offset":57728,"inlinee":77132,"source_line":820,"archive":"_pdb/B/modules/04679.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4679,"record_offset":58052,"parent_record_offset":58028,"ancestor_record_offset":57728,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/04679.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4679,"record_offset":58124,"parent_record_offset":58100,"ancestor_record_offset":57728,"inlinee":5241,"source_line":413,"archive":"_pdb/B/modules/04679.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4679,"record_offset":58100,"parent_record_offset":58076,"ancestor_record_offset":57728,"inlinee":5240,"source_line":553,"archive":"_pdb/B/modules/04679.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4679,"record_offset":58076,"parent_record_offset":58052,"ancestor_record_offset":57728,"inlinee":5239,"source_line":568,"archive":"_pdb/B/modules/04679.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zeno::raster::Rasterizer::curve_to<zeno::raster::HeapStorage>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4679,"record_offset":57288,"rva":42118720,"va":5410827840}
// 编译实例 RVAs: 0x282ae40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14282a070
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410827840,"raw_hex":"e876f1ffff","size":5,"soft_fail":false,"target_va":5410824304,"va":5410828021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t714000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410827840,"raw_hex":"e810e50a00","size":5,"soft_fail":false,"target_va":5411542296,"va":5410828291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::raster::split_cubic
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-17371735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410827840,"raw_hex":"e8a9edf6fe","size":5,"soft_fail":false,"target_va":5393456528,"va":5410828258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::impl$16::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4679,"record_offset":57584,"parent_record_offset":57288,"ancestor_record_offset":57288,"inlinee":10912,"source_line":374,"archive":"_pdb/B/modules/04679.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$2::abs
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":4679,"record_offset":57472,"parent_record_offset":57288,"ancestor_record_offset":57288,"inlinee":14836,"source_line":3449,"archive":"_pdb/B/modules/04679.jsonl.gz"}
//   [pdb_inline_direct] zeno::raster::trunc
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/raster.rs
//     evidence={"module":4679,"record_offset":57444,"parent_record_offset":57288,"ancestor_record_offset":57288,"inlinee":77132,"source_line":820,"archive":"_pdb/B/modules/04679.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4679,"record_offset":57608,"parent_record_offset":57584,"ancestor_record_offset":57288,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/04679.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4679,"record_offset":57680,"parent_record_offset":57656,"ancestor_record_offset":57288,"inlinee":5241,"source_line":413,"archive":"_pdb/B/modules/04679.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4679,"record_offset":57656,"parent_record_offset":57632,"ancestor_record_offset":57288,"inlinee":5240,"source_line":553,"archive":"_pdb/B/modules/04679.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4679,"record_offset":57632,"parent_record_offset":57608,"ancestor_record_offset":57288,"inlinee":5239,"source_line":568,"archive":"_pdb/B/modules/04679.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zeno::segment::Segments::split_curve<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3650,"record_offset":260,"rva":36057840,"va":5404766960}
// 编译实例 RVAs: 0x22632f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t6774259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404766960,"raw_hex":"e8f35d6700","size":5,"soft_fail":false,"target_va":5411542296,"va":5404768032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::is_line
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-1674186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404766960,"raw_hex":"e83674e6ff","size":5,"soft_fail":false,"target_va":5403092832,"va":5404767013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::needs_split
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-1670962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404766960,"raw_hex":"e8ce80e6ff","size":5,"soft_fail":false,"target_va":5403096368,"va":5404767325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::split
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-1672539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404766960,"raw_hex":"e8a57ae6ff","size":5,"soft_fail":false,"target_va":5403094816,"va":5404767350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::split_at_max_curvature
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-1673872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404766960,"raw_hex":"e87075e6ff","size":5,"soft_fail":false,"target_va":5403093312,"va":5404767179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::to_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-1674995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404766960,"raw_hex":"e80d71e6ff","size":5,"soft_fail":false,"target_va":5403092992,"va":5404767982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3650,"record_offset":544,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] zeno::geometry::Vector::nearly_eq_by
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3650,"record_offset":484,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":75821,"source_line":130,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3650,"record_offset":600,"parent_record_offset":572,"ancestor_record_offset":260,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3650,"record_offset":508,"parent_record_offset":484,"ancestor_record_offset":260,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3650,"record_offset":572,"parent_record_offset":544,"ancestor_record_offset":260,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zeno::segment::Segments::split_curve<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3650,"record_offset":644,"rva":36059072,"va":5404768192}
// 编译实例 RVAs: 0x22637c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t6773027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404768192,"raw_hex":"e823596700","size":5,"soft_fail":false,"target_va":5411542296,"va":5404769264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::is_line
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-1675418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404768192,"raw_hex":"e8666fe6ff","size":5,"soft_fail":false,"target_va":5403092832,"va":5404768245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::needs_split
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-1672194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404768192,"raw_hex":"e8fe7be6ff","size":5,"soft_fail":false,"target_va":5403096368,"va":5404768557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::split
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-1673771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404768192,"raw_hex":"e8d575e6ff","size":5,"soft_fail":false,"target_va":5403094816,"va":5404768582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::split_at_max_curvature
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-1675104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404768192,"raw_hex":"e8a070e6ff","size":5,"soft_fail":false,"target_va":5403093312,"va":5404768411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::to_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-1676227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404768192,"raw_hex":"e83d6ce6ff","size":5,"soft_fail":false,"target_va":5403092992,"va":5404769214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3650,"record_offset":1140,"parent_record_offset":644,"ancestor_record_offset":644,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] zeno::geometry::Vector::nearly_eq_by
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3650,"record_offset":1080,"parent_record_offset":644,"ancestor_record_offset":644,"inlinee":75821,"source_line":130,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3650,"record_offset":1196,"parent_record_offset":1168,"ancestor_record_offset":644,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3650,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":644,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3650,"record_offset":1168,"parent_record_offset":1140,"ancestor_record_offset":644,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zeno::segment::impl$5::next<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3650,"record_offset":4744,"rva":36063888,"va":5404773008}
// 编译实例 RVAs: 0x2264a90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6767762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404773008,"raw_hex":"e892446700","size":5,"soft_fail":false,"target_va":5411542296,"va":5404774529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::to_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-1680305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404773008,"raw_hex":"e84f5ce6ff","size":5,"soft_fail":false,"target_va":5403092992,"va":5404773292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Segments::split_curve<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > > >
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-6243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404773008,"raw_hex":"e89de7ffff","size":5,"soft_fail":false,"target_va":5404766960,"va":5404773198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":3650,"record_offset":5040,"parent_record_offset":4744,"ancestor_record_offset":4744,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3650,"record_offset":5256,"parent_record_offset":4744,"ancestor_record_offset":4744,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$40::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3650,"record_offset":5840,"parent_record_offset":4744,"ancestor_record_offset":4744,"inlinee":6199,"source_line":2499,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] zeno::geometry::Vector::nearly_eq_by
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3650,"record_offset":5284,"parent_record_offset":4744,"ancestor_record_offset":4744,"inlinee":75821,"source_line":130,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Curve::from_quadratic
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3650,"record_offset":4980,"parent_record_offset":4744,"ancestor_record_offset":4744,"inlinee":75841,"source_line":88,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Curve::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3650,"record_offset":5344,"parent_record_offset":4744,"ancestor_record_offset":4744,"inlinee":75838,"source_line":73,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Segments::inc_id
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3650,"record_offset":5012,"parent_record_offset":4744,"ancestor_record_offset":4744,"inlinee":75840,"source_line":494,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3650,"record_offset":5308,"parent_record_offset":5284,"ancestor_record_offset":4744,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3650,"record_offset":5220,"parent_record_offset":5040,"ancestor_record_offset":4744,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3650,"record_offset":5180,"parent_record_offset":5156,"ancestor_record_offset":4744,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3650,"record_offset":5104,"parent_record_offset":5068,"ancestor_record_offset":4744,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3650,"record_offset":5156,"parent_record_offset":5132,"ancestor_record_offset":4744,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3650,"record_offset":5068,"parent_record_offset":5040,"ancestor_record_offset":4744,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3650,"record_offset":5132,"parent_record_offset":5068,"ancestor_record_offset":4744,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zeno::segment::impl$5::next<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3650,"record_offset":2968,"rva":36062096,"va":5404771216}
// 编译实例 RVAs: 0x2264390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6769299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404771216,"raw_hex":"e8934a6700","size":5,"soft_fail":false,"target_va":5411542296,"va":5404772992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::command::Command::transform::h8a4c410fe33a1967 (.llvm.2455388611895484099)
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t3643309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404771216,"raw_hex":"e8ad973700","size":5,"soft_fail":false,"target_va":5408414976,"va":5404771662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::to_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-1678545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404771216,"raw_hex":"e82f63e6ff","size":5,"soft_fail":false,"target_va":5403092992,"va":5404771532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Segments::split_curve<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > >
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-3238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404771216,"raw_hex":"e85af3ffff","size":5,"soft_fail":false,"target_va":5404768192,"va":5404771425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3650,"record_offset":3468,"parent_record_offset":2968,"ancestor_record_offset":2968,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3650,"record_offset":3792,"parent_record_offset":2968,"ancestor_record_offset":2968,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$40::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3650,"record_offset":4480,"parent_record_offset":2968,"ancestor_record_offset":2968,"inlinee":6199,"source_line":2499,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] zeno::geometry::Vector::nearly_eq_by
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3650,"record_offset":3860,"parent_record_offset":2968,"ancestor_record_offset":2968,"inlinee":75821,"source_line":130,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Curve::from_quadratic
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3650,"record_offset":3824,"parent_record_offset":2968,"ancestor_record_offset":2968,"inlinee":75841,"source_line":88,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Curve::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3650,"record_offset":3412,"parent_record_offset":2968,"ancestor_record_offset":2968,"inlinee":75838,"source_line":73,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Segments::inc_id
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3650,"record_offset":3440,"parent_record_offset":2968,"ancestor_record_offset":2968,"inlinee":75840,"source_line":494,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3650,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":2968,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":3650,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":2968,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3650,"record_offset":3728,"parent_record_offset":3700,"ancestor_record_offset":2968,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3650,"record_offset":3668,"parent_record_offset":3492,"ancestor_record_offset":2968,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3650,"record_offset":3700,"parent_record_offset":3468,"ancestor_record_offset":2968,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3650,"record_offset":3628,"parent_record_offset":3604,"ancestor_record_offset":2968,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3650,"record_offset":3552,"parent_record_offset":3516,"ancestor_record_offset":2968,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3650,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":2968,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3650,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":2968,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3650,"record_offset":3580,"parent_record_offset":3516,"ancestor_record_offset":2968,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] zeno::path_data::apply::closure$0
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/path_data.rs
//     evidence={"module":3650,"record_offset":3752,"parent_record_offset":3728,"ancestor_record_offset":2968,"inlinee":30043,"source_line":164,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zeno::segment::impl$5::next<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::geometry::BoundsBuilder> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3650,"record_offset":1240,"rva":36060304,"va":5404769424}
// 编译实例 RVAs: 0x2263c90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6771091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404769424,"raw_hex":"e893516700","size":5,"soft_fail":false,"target_va":5411542296,"va":5404771200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::command::Command::transform::h8a4c410fe33a1967 (.llvm.15262956913503184486)
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t3643965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404769424,"raw_hex":"e83d9a3700","size":5,"soft_fail":false,"target_va":5408413840,"va":5404769870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::to_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-1676753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404769424,"raw_hex":"e82f6ae6ff","size":5,"soft_fail":false,"target_va":5403092992,"va":5404769740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Segments::split_curve<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > >
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-1446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404769424,"raw_hex":"e85afaffff","size":5,"soft_fail":false,"target_va":5404768192,"va":5404769633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3650,"record_offset":1692,"parent_record_offset":1240,"ancestor_record_offset":1240,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3650,"record_offset":2016,"parent_record_offset":1240,"ancestor_record_offset":1240,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$40::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3650,"record_offset":2704,"parent_record_offset":1240,"ancestor_record_offset":1240,"inlinee":6199,"source_line":2499,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] zeno::geometry::Vector::nearly_eq_by
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3650,"record_offset":2084,"parent_record_offset":1240,"ancestor_record_offset":1240,"inlinee":75821,"source_line":130,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Curve::from_quadratic
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3650,"record_offset":2048,"parent_record_offset":1240,"ancestor_record_offset":1240,"inlinee":75841,"source_line":88,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Curve::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3650,"record_offset":1636,"parent_record_offset":1240,"ancestor_record_offset":1240,"inlinee":75838,"source_line":73,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Segments::inc_id
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3650,"record_offset":1664,"parent_record_offset":1240,"ancestor_record_offset":1240,"inlinee":75840,"source_line":494,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3650,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1240,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":3650,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":1240,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3650,"record_offset":1952,"parent_record_offset":1924,"ancestor_record_offset":1240,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3650,"record_offset":1892,"parent_record_offset":1716,"ancestor_record_offset":1240,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3650,"record_offset":1924,"parent_record_offset":1692,"ancestor_record_offset":1240,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3650,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":1240,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3650,"record_offset":1776,"parent_record_offset":1740,"ancestor_record_offset":1240,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3650,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":1240,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3650,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":1240,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3650,"record_offset":1804,"parent_record_offset":1740,"ancestor_record_offset":1240,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   [pdb_inline_transitive] zeno::scratch::impl$1::apply::closure$0
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/scratch.rs
//     evidence={"module":3650,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":1240,"inlinee":72488,"source_line":89,"archive":"_pdb/B/modules/03650.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_water::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":302,"record_offset":1472,"rva":6619600,"va":5375328720}
// 编译实例 RVAs: 0x6501d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140029880
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6450272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375328720,"raw_hex":"e8a0939dff","size":5,"soft_fail":false,"target_va":5368879232,"va":5375329499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=3
//     target_aliases=19 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5713077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375328720,"raw_hex":"e84bd3a8ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5375329184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14083eaf0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2025340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375328720,"raw_hex":"e87ce71e00","size":5,"soft_fail":false,"target_va":5377354480,"va":5375329135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e92740
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8660282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375328720,"raw_hex":"e83a258400","size":5,"soft_fail":false,"target_va":5383989056,"va":5375328769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e930c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8662693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375328720,"raw_hex":"e8a52e8400","size":5,"soft_fail":false,"target_va":5383991488,"va":5375328790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e94020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8666454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375328720,"raw_hex":"e8563d8400","size":5,"soft_fail":false,"target_va":5383995424,"va":5375328965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8672561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375328720,"raw_hex":"e831558400","size":5,"soft_fail":false,"target_va":5384001712,"va":5375329146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8663759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375328720,"raw_hex":"e8cf328400","size":5,"soft_fail":false,"target_va":5383992720,"va":5375328956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8674315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375328720,"raw_hex":"e80b5c8400","size":5,"soft_fail":false,"target_va":5384003568,"va":5375329248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8668853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375328720,"raw_hex":"e8b5468400","size":5,"soft_fail":false,"target_va":5383997856,"va":5375328998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::ScheduleConfigs$LT$T$GT$::in_set_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8672550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375328720,"raw_hex":"e826558400","size":5,"soft_fail":false,"target_va":5384001568,"va":5375329013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::world_raster::DirtyTilePublisher::unique
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3051595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375328720,"raw_hex":"e84b902e00","size":5,"soft_fail":false,"target_va":5378380720,"va":5375329120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t8677276","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375328720,"raw_hex":"e99c678400","size":5,"soft_fail":false,"target_va":5384006736,"va":5375329455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":302,"record_offset":1720,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":9230,"source_line":358,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":302,"record_offset":1592,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/schedule/config.rs
//     evidence={"module":302,"record_offset":1620,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":7908,"source_line":492,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":302,"record_offset":2188,"parent_record_offset":2164,"ancestor_record_offset":1472,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":302,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":1472,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":302,"record_offset":2212,"parent_record_offset":2188,"ancestor_record_offset":1472,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":302,"record_offset":2164,"parent_record_offset":2140,"ancestor_record_offset":1472,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":302,"record_offset":2140,"parent_record_offset":2108,"ancestor_record_offset":1472,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":302,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1472,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":302,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":1472,"inlinee":9217,"source_line":339,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/label.rs
//     evidence={"module":302,"record_offset":1660,"parent_record_offset":1620,"ancestor_record_offset":1472,"inlinee":5551,"source_line":139,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":302,"record_offset":2380,"parent_record_offset":2356,"ancestor_record_offset":1472,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":302,"record_offset":2356,"parent_record_offset":2332,"ancestor_record_offset":1472,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":302,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":1472,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":302,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":1472,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":302,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":1472,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":302,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":1472,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":302,"record_offset":1772,"parent_record_offset":1744,"ancestor_record_offset":1472,"inlinee":9218,"source_line":1825,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":302,"record_offset":1832,"parent_record_offset":1804,"ancestor_record_offset":1472,"inlinee":9220,"source_line":1828,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":302,"record_offset":1996,"parent_record_offset":1932,"ancestor_record_offset":1472,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":302,"record_offset":1804,"parent_record_offset":1772,"ancestor_record_offset":1472,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":302,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":1472,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":302,"record_offset":2308,"parent_record_offset":1772,"ancestor_record_offset":1472,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":302,"record_offset":2060,"parent_record_offset":1880,"ancestor_record_offset":1472,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":302,"record_offset":1932,"parent_record_offset":1904,"ancestor_record_offset":1472,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00302.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":302,"record_offset":1968,"parent_record_offset":1932,"ancestor_record_offset":1472,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00302.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] system_water::WaterPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<system_water::WaterPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["system_water::WaterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<system_water::WaterPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["system_water::WaterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<system_water::WaterPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["system_water::WaterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<system_water::WaterPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["system_water::WaterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<system_water::WaterPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["system_water::WaterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<system_water::WaterPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["system_water::WaterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<system_water::WaterPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["system_water::WaterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<system_water::WaterPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["system_water::WaterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$16::add_to_app<bevy_app::plugin::sealed::PluginMarker,feedback_reporter::FeedbackReporterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::menu_background::MenuBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::paths::PathPlugin,bevy_app::plugin::sealed::PluginMarker,system_water::WaterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::garden::GardenPlugin,bevy_app::plugin::sealed::PluginMarker,system_tree::TreePlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::prefab::PrefabPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::photo_mode::PhotoModePlugin>", "constructor": "bevy_app::plugin::sealed::impl$16::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "feedback_reporter::FeedbackReporterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::menu_background::MenuBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::paths::PathPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_water::WaterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::garden::GardenPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_tree::TreePlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::prefab::PrefabPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::vermeer::VermeerPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::photo_mode::PhotoModePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::event::reader::EventReader<system_water::record_water_history::RecordWaterEditToHistoryCmd>)
// 候选参数类型: ["country_core::systems::world_raster::RasterHistoryRecordResources", "bevy_ecs::event::reader::EventReader<system_water::record_water_history::RecordWaterEditToHistoryCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<system_water::record_water_history::RecordWaterEditToHistoryCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterEraseEvent>,bevy_ecs::event::writer::EventWriter<system_water::record_water_history::RecordWaterEditToHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "country_core::systems::world_raster::RasterDrawResources", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>", "bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterEraseEvent>", "bevy_ecs::event::writer::EventWriter<system_water::record_water_history::RecordWaterEditToHistoryCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>", "bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes> >", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterEraseEvent>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_water::record_water_history::RecordWaterEditToHistoryCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::system_param::Local<system_water::water_audio_grid::WaterAudioGridUpdateState>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>)
// 候选参数类型: ["bevy_ecs::system::system_param::Local<system_water::water_audio_grid::WaterAudioGridUpdateState>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "country_core::systems::collision::world::Raycasting", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>"]
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_water::water_audio_grid::WaterAudioGridUpdateState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>

// ------------------------------------------------------------------
// 函数: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 信息 ID: 935fd15b7fefbfb868c4a7b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["core::str::pattern::MatchOnly"]; 字段布局 = unknown。
// [类型关系] core::str::pattern::TwoWaySearcher::next_back -> ["enum2$<core::str::pattern::MatchOnly>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1baed57 -> 0x1baeda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x2262fd7 -> 0x2263020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_water::impl$0::build
// 信息 ID: 538bf5ae6a092611bb8559c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zeno::raster::Rasterizer::curve_to<zeno::raster::AdaptiveStorage>
// 信息 ID: 3681b6d932ea548027ea5d14
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] zeno::raster::Rasterizer::curve_to -> ["zeno::raster::AdaptiveStorage"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x2834504 -> 0x282b030 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x2833ff8 -> 0x282b030 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x2836b74 -> 0x282b030 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x2836668 -> 0x282b030 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > @ 0x2838c3e -> 0x282b030 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > @ 0x28393e1 -> 0x282b030 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::Stroker::stroke_segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x2849c8c -> 0x282b030 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::Stroker::stroke_segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x284a12e -> 0x282b030 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::Stroker::stroke_segments<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x284aedc -> 0x282b030 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::Stroker::stroke_segments<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x284b37e -> 0x282b030 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::Stroker::stroke_segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > @ 0x284ca7e -> 0x282b030 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::Stroker::stroke_segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > @ 0x284d575 -> 0x282b030 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::Stroker::stroke_segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > @ 0x284d15d -> 0x282b030 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::path_builder::arc<zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > @ 0x288612d -> 0x282b030 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::path_builder::arc<zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x2887521 -> 0x282b030 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zeno::raster::Rasterizer::curve_to<zeno::raster::HeapStorage>
// 信息 ID: 648763bb0bdaf48bfa0fe018
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] zeno::raster::Rasterizer::curve_to -> ["zeno::raster::HeapStorage"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::HeapStorage> >,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x283d3cd -> 0x282ae40 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::HeapStorage> >,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x283cd3f -> 0x282ae40 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x283ed6b -> 0x282ae40 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x283e8a9 -> 0x282ae40 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x2842bfe -> 0x282ae40 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x284273c -> 0x282ae40 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::Stroker::stroke_segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > @ 0x2847639 -> 0x282ae40 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::Stroker::stroke_segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > @ 0x2847a6e -> 0x282ae40 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::Stroker::stroke_segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::HeapStorage> > > @ 0x284813e -> 0x282ae40 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::Stroker::stroke_segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::HeapStorage> > > @ 0x2848c35 -> 0x282ae40 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::Stroker::stroke_segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::HeapStorage> > > @ 0x284881d -> 0x282ae40 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::Stroker::stroke_segments<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > @ 0x284bf79 -> 0x282ae40 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::Stroker::stroke_segments<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > @ 0x284c3ae -> 0x282ae40 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::path_builder::arc<zeno::raster::Rasterizer<zeno::raster::HeapStorage> > @ 0x2886771 -> 0x282ae40 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::path_builder::arc<zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::HeapStorage> > > @ 0x2886edd -> 0x282ae40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zeno::segment::Segments::split_curve<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > > >
// 信息 ID: ff5ad368479050c48bd235ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["zeno::command::Command"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["enum2$<zeno::command::Command>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::copied::Copied -> ["core::slice::iter::Iter<enum2$<zeno::command::Command> >"]; 字段布局 = unknown。
// [类型关系] zeno::segment::Segments::split_curve -> ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zeno::segment::Segments::split_curve<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > >
// 信息 ID: 6552cf96e5d305483ad83d65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["zeno::command::Command"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["enum2$<zeno::command::Command>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::copied::Copied -> ["core::slice::iter::Iter<enum2$<zeno::command::Command> >"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["zeno::command::Command"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["enum2$<zeno::command::Command>"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<enum2$<zeno::command::Command> >"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["ref$<slice2$<enum2$<zeno::command::Command> > >"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["zeno::style::Style"]; 字段布局 = unknown。
// [类型关系] zeno::raster::Rasterizer -> ["zeno::raster::AdaptiveStorage"]; 字段布局 = unknown。
// [类型关系] zeno::path_data::apply::closure_env$0 -> ["ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >", "enum2$<zeno::style::Style>", "zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >", "zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> >"]; 字段布局 = unknown。
// [类型关系] zeno::segment::Segments::split_curve -> ["core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zeno::segment::impl$5::next<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > > >
// 信息 ID: 4d7e3ef9efcdd8dcab777708
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["zeno::command::Command"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["enum2$<zeno::command::Command>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::copied::Copied -> ["core::slice::iter::Iter<enum2$<zeno::command::Command> >"]; 字段布局 = unknown。
// [类型关系] zeno::segment::impl$5::next -> ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 20 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::geometry::BoundsBuilder> > @ 0x282facd -> 0x2264a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::geometry::BoundsBuilder> > @ 0x2830297 -> 0x2264a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x2833017 -> 0x2264a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x2834cb4 -> 0x2264a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > @ 0x2837ce7 -> 0x2264a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > @ 0x2839a31 -> 0x2264a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::geometry::BoundsBuilder> @ 0x283a2fd -> 0x2264a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::geometry::BoundsBuilder> @ 0x283aac7 -> 0x2264a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::geometry::BoundsBuilder>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x283b210 -> 0x2264a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::geometry::BoundsBuilder>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x283b921 -> 0x2264a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::geometry::BoundsBuilder>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x283ba20 -> 0x2264a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::HeapStorage> >,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x283bcf0 -> 0x2264a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::HeapStorage> >,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x283d701 -> 0x2264a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::HeapStorage> >,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x283d822 -> 0x2264a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x28419c0 -> 0x2264a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x28432d6 -> 0x2264a90 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zeno::segment::impl$5::next<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > >
// 信息 ID: 5b91e3dc4dfa439ea068bf43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["zeno::command::Command"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["enum2$<zeno::command::Command>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::copied::Copied -> ["core::slice::iter::Iter<enum2$<zeno::command::Command> >"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["zeno::command::Command"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["enum2$<zeno::command::Command>"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<enum2$<zeno::command::Command> >"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["ref$<slice2$<enum2$<zeno::command::Command> > >"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["zeno::style::Style"]; 字段布局 = unknown。
// [类型关系] zeno::raster::Rasterizer -> ["zeno::raster::AdaptiveStorage"]; 字段布局 = unknown。
// [类型关系] zeno::path_data::apply::closure_env$0 -> ["ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >", "enum2$<zeno::style::Style>", "zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >", "zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> >"]; 字段布局 = unknown。
// [类型关系] zeno::segment::impl$5::next -> ["core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::geometry::BoundsBuilder> >,zeno::geometry::BoundsBuilder> @ 0x2830a56 -> 0x2264390 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::geometry::BoundsBuilder> >,zeno::geometry::BoundsBuilder> @ 0x2832711 -> 0x2264390 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x2835687 -> 0x2264390 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x2837324 -> 0x2264390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zeno::segment::impl$5::next<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::geometry::BoundsBuilder> > >
// 信息 ID: f6a3084ff8b5d2bff6d443d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["zeno::command::Command"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["enum2$<zeno::command::Command>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::copied::Copied -> ["core::slice::iter::Iter<enum2$<zeno::command::Command> >"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["zeno::command::Command"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["enum2$<zeno::command::Command>"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<enum2$<zeno::command::Command> >"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["ref$<slice2$<enum2$<zeno::command::Command> > >"]; 字段布局 = unknown。
// [类型关系] zeno::scratch::impl$1::apply::closure_env$0 -> ["ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >", "zeno::geometry::BoundsBuilder"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >", "zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::geometry::BoundsBuilder>"]; 字段布局 = unknown。
// [类型关系] zeno::segment::impl$5::next -> ["core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::geometry::BoundsBuilder> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x283daf2 -> 0x2263c90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x283f44b -> 0x2263c90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x283f56c -> 0x2263c90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::geometry::BoundsBuilder> >,zeno::geometry::BoundsBuilder,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x283f84d -> 0x2263c90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::geometry::BoundsBuilder> >,zeno::geometry::BoundsBuilder,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x28415ec -> 0x2263c90 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::geometry::BoundsBuilder> >,zeno::geometry::BoundsBuilder,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x28416fc -> 0x2263c90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
