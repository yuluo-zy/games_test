// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tg-text/src/shape_stream.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b940e44fb0623989fdba29fed3016ec86a076fddc80faa41eec50e5ac98a11cb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shape_stream.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 342, 626, 1186, 1211, 2778, 3942
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 7, 51, 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1D9
// 首条位置: {"module":2778,"inlinee":61913}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":80044,"parent_record_offset":79960,"ancestor_record_offset":79960,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":2778,"record_offset":16004,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":2778,"record_offset":79988,"parent_record_offset":79960,"ancestor_record_offset":79960,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_seq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":2778,"record_offset":80012,"parent_record_offset":79988,"ancestor_record_offset":79960,"inlinee":6904,"source_line":188,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$8::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4EDC
// 首条位置: {"module":342,"inlinee":20188}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":626,"record_offset":256,"rva":11021088,"va":5379730208}
// 编译实例 RVAs: 0xa82b20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::fill_color
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":626,"record_offset":372,"rva":12132528,"va":5380841648}
// 编译实例 RVAs: 0xb920b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f11f0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5902170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380841648,"raw_hex":"e8a6f0a5ff","size":5,"soft_fail":false,"target_va":5374939632,"va":5380841797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6485176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380841648,"raw_hex":"e8480b9dff","size":5,"soft_fail":false,"target_va":5374356528,"va":5380841699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":504,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":572,"parent_record_offset":540,"ancestor_record_offset":372,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":620,"parent_record_offset":596,"ancestor_record_offset":372,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":596,"parent_record_offset":572,"ancestor_record_offset":372,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":540,"parent_record_offset":504,"ancestor_record_offset":372,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":626,"record_offset":660,"parent_record_offset":504,"ancestor_record_offset":372,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::image<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1186,"record_offset":1944,"rva":16438944,"va":5385148064}
// 编译实例 RVAs: 0xfad6a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f11f0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10208746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385148064,"raw_hex":"e8163a64ff","size":5,"soft_fail":false,"target_va":5374939632,"va":5385148373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10066827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385148064,"raw_hex":"e8756466ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5385148326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b922e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4305899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385148064,"raw_hex":"e8154cbeff","size":5,"soft_fail":false,"target_va":5380842208,"va":5385148102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10791628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385148064,"raw_hex":"e834555bff","size":5,"soft_fail":false,"target_va":5374356528,"va":5385148151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1186,"record_offset":2116,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1186,"record_offset":2088,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1186,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":1944,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1186,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":1944,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1186,"record_offset":2192,"parent_record_offset":2160,"ancestor_record_offset":1944,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1186,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":1944,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1186,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":1944,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1186,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":1944,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1186,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":1944,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1186,"record_offset":2160,"parent_record_offset":2116,"ancestor_record_offset":1944,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1186,"record_offset":2308,"parent_record_offset":2116,"ancestor_record_offset":1944,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1186,"record_offset":2280,"parent_record_offset":2116,"ancestor_record_offset":1944,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::image<u32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1186,"record_offset":1300,"rva":16438608,"va":5385147728}
// 编译实例 RVAs: 0xfad550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f11f0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10208412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385147728,"raw_hex":"e8643b64ff","size":5,"soft_fail":false,"target_va":5374939632,"va":5385148039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10066493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385147728,"raw_hex":"e8c36566ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5385147992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b922c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4305597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385147728,"raw_hex":"e8434dbeff","size":5,"soft_fail":false,"target_va":5380842176,"va":5385147768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10791294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385147728,"raw_hex":"e882565bff","size":5,"soft_fail":false,"target_va":5374356528,"va":5385147817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1186,"record_offset":1464,"parent_record_offset":1300,"ancestor_record_offset":1300,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1186,"record_offset":1436,"parent_record_offset":1300,"ancestor_record_offset":1300,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1186,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":1300,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1186,"record_offset":1848,"parent_record_offset":1824,"ancestor_record_offset":1300,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1186,"record_offset":1540,"parent_record_offset":1508,"ancestor_record_offset":1300,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1186,"record_offset":1824,"parent_record_offset":1800,"ancestor_record_offset":1300,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1186,"record_offset":1588,"parent_record_offset":1564,"ancestor_record_offset":1300,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1186,"record_offset":1564,"parent_record_offset":1540,"ancestor_record_offset":1300,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1186,"record_offset":1800,"parent_record_offset":1776,"ancestor_record_offset":1300,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1186,"record_offset":1508,"parent_record_offset":1464,"ancestor_record_offset":1300,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1186,"record_offset":1656,"parent_record_offset":1464,"ancestor_record_offset":1300,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1186,"record_offset":1628,"parent_record_offset":1464,"ancestor_record_offset":1300,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::corner_radius
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":626,"record_offset":696,"rva":12132704,"va":5380841824}
// 编译实例 RVAs: 0xb92160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f11f0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5902350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380841824,"raw_hex":"e8f2efa5ff","size":5,"soft_fail":false,"target_va":5374939632,"va":5380841977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6485355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380841824,"raw_hex":"e8950a9dff","size":5,"soft_fail":false,"target_va":5374356528,"va":5380841878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":832,"parent_record_offset":696,"ancestor_record_offset":696,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":900,"parent_record_offset":868,"ancestor_record_offset":696,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":696,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":924,"parent_record_offset":900,"ancestor_record_offset":696,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":868,"parent_record_offset":832,"ancestor_record_offset":696,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":626,"record_offset":988,"parent_record_offset":832,"ancestor_record_offset":696,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::is_empty
// 记录形式: compiled_procedure; instances=1
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":626,"record_offset":1024,"rva":12132880,"va":5380842000}
// 编译实例 RVAs: 0xb92210
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":1152,"parent_record_offset":1024,"ancestor_record_offset":1024,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::encode_no_images
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":626,"record_offset":1180,"rva":12132896,"va":5380842016}
// 编译实例 RVAs: 0xb92220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5760651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380842016,"raw_hex":"e87519a8ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380842150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::RawShapeStream::append
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380842016,"raw_hex":"e8dc000000","size":5,"soft_fail":false,"target_va":5380842304,"va":5380842079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":626,"record_offset":1432,"parent_record_offset":1180,"ancestor_record_offset":1180,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_direct] tg_text::shape_stream::RawShapeStream::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":626,"record_offset":1376,"parent_record_offset":1180,"ancestor_record_offset":1180,"inlinee":34508,"source_line":162,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_direct] tg_text::shape_stream::RawShapeStream::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":626,"record_offset":1324,"parent_record_offset":1180,"ancestor_record_offset":1180,"inlinee":34505,"source_line":113,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":626,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":1180,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":626,"record_offset":1604,"parent_record_offset":1516,"ancestor_record_offset":1180,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":1544,"parent_record_offset":1516,"ancestor_record_offset":1180,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":1180,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":1492,"parent_record_offset":1472,"ancestor_record_offset":1180,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":1400,"parent_record_offset":1376,"ancestor_record_offset":1180,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":626,"record_offset":1572,"parent_record_offset":1544,"ancestor_record_offset":1180,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":626,"record_offset":1452,"parent_record_offset":1432,"ancestor_record_offset":1180,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::impl$22::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":626,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":1180,"inlinee":34507,"source_line":95,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1186,"record_offset":656,"rva":16438416,"va":5385147536}
// 编译实例 RVAs: 0xfad490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26333435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385147536,"raw_hex":"e8fbd09101","size":5,"soft_fail":false,"target_va":5411481152,"va":5385147712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1186,"record_offset":832,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1186,"record_offset":852,"parent_record_offset":832,"ancestor_record_offset":656,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1186,"record_offset":872,"parent_record_offset":852,"ancestor_record_offset":656,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1186,"record_offset":892,"parent_record_offset":872,"ancestor_record_offset":656,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 51, 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x186DF
// 首条位置: {"module":2778,"inlinee":100063}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":2778,"record_offset":10464,"parent_record_offset":10440,"ancestor_record_offset":10440,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2778,"record_offset":11224,"parent_record_offset":11196,"ancestor_record_offset":11020,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":10500,"parent_record_offset":10464,"ancestor_record_offset":10440,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":10564,"parent_record_offset":10464,"ancestor_record_offset":10440,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":11252,"parent_record_offset":11224,"ancestor_record_offset":11020,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":11280,"parent_record_offset":11252,"ancestor_record_offset":11020,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2778,"record_offset":11196,"parent_record_offset":11164,"ancestor_record_offset":11020,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2778,"record_offset":10536,"parent_record_offset":10464,"ancestor_record_offset":10440,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":2778,"record_offset":11164,"parent_record_offset":11132,"ancestor_record_offset":11020,"inlinee":61909,"source_line":85,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1D4
// 首条位置: {"module":2778,"inlinee":61908}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":63876,"parent_record_offset":63820,"ancestor_record_offset":63820,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2778,"record_offset":63848,"parent_record_offset":63820,"ancestor_record_offset":63820,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize<ref_mut$<ron::de::tag::Deserializer> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1186,"record_offset":2596,"rva":16439280,"va":5385148400}
// 编译实例 RVAs: 0xfad7f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140681f40
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9615601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385148400,"raw_hex":"e80f476dff","size":5,"soft_fail":false,"target_va":5375532864,"va":5385148460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e43360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15293272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385148400,"raw_hex":"e8585be900","size":5,"soft_fail":false,"target_va":5400441696,"va":5385148419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1186,"record_offset":2852,"parent_record_offset":2596,"ancestor_record_offset":2596,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1186,"record_offset":2768,"parent_record_offset":2596,"ancestor_record_offset":2596,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1186,"record_offset":2880,"parent_record_offset":2852,"ancestor_record_offset":2596,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":1186,"record_offset":2816,"parent_record_offset":2792,"ancestor_record_offset":2596,"inlinee":5162,"source_line":455,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":1186,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":2596,"inlinee":5160,"source_line":139,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$19::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF32A
// 首条位置: {"module":1211,"inlinee":62250}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::borrow::impl$2::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":1211,"record_offset":15844,"parent_record_offset":15812,"ancestor_record_offset":15812,"inlinee":4241,"source_line":194,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1211,"record_offset":16344,"parent_record_offset":16320,"ancestor_record_offset":15812,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1211,"record_offset":16368,"parent_record_offset":16344,"ancestor_record_offset":15812,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1211,"record_offset":16320,"parent_record_offset":16144,"ancestor_record_offset":15812,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1211,"record_offset":16084,"parent_record_offset":16056,"ancestor_record_offset":15812,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1211,"record_offset":16144,"parent_record_offset":16112,"ancestor_record_offset":15812,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1211,"record_offset":16112,"parent_record_offset":16084,"ancestor_record_offset":15812,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1211,"record_offset":16180,"parent_record_offset":16144,"ancestor_record_offset":15812,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1211,"record_offset":16020,"parent_record_offset":15992,"ancestor_record_offset":15812,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1211,"record_offset":15992,"parent_record_offset":15964,"ancestor_record_offset":15812,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1211,"record_offset":15936,"parent_record_offset":15908,"ancestor_record_offset":15812,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1211,"record_offset":15964,"parent_record_offset":15936,"ancestor_record_offset":15812,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1211,"record_offset":15908,"parent_record_offset":15880,"ancestor_record_offset":15812,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1211,"record_offset":15880,"parent_record_offset":15844,"ancestor_record_offset":15812,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1211,"record_offset":16056,"parent_record_offset":16020,"ancestor_record_offset":15812,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1211,"record_offset":16204,"parent_record_offset":16180,"ancestor_record_offset":15812,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1211,"record_offset":16228,"parent_record_offset":16204,"ancestor_record_offset":15812,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1211,"record_offset":16444,"parent_record_offset":16420,"ancestor_record_offset":15812,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1211,"record_offset":16276,"parent_record_offset":16252,"ancestor_record_offset":15812,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1211,"record_offset":16252,"parent_record_offset":16228,"ancestor_record_offset":15812,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1211,"record_offset":16420,"parent_record_offset":16020,"ancestor_record_offset":15812,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$1::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":626,"record_offset":1684,"rva":12133056,"va":5380842176}
// 编译实例 RVAs: 0xb922c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$2::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":626,"record_offset":1796,"rva":12133088,"va":5380842208}
// 编译实例 RVAs: 0xb922e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$3::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":626,"record_offset":1908,"rva":6823744,"va":5375532864}
// 编译实例 RVAs: 0x681f40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1D5
// 首条位置: {"module":2778,"inlinee":61909}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2778,"record_offset":11196,"parent_record_offset":11164,"ancestor_record_offset":11164,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2778,"record_offset":11224,"parent_record_offset":11196,"ancestor_record_offset":11164,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":11252,"parent_record_offset":11224,"ancestor_record_offset":11164,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":11280,"parent_record_offset":11252,"ancestor_record_offset":11164,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1D6
// 首条位置: {"module":2778,"inlinee":61910}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2778,"record_offset":15336,"parent_record_offset":15308,"ancestor_record_offset":15308,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2778,"record_offset":15360,"parent_record_offset":15336,"ancestor_record_offset":15308,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":15384,"parent_record_offset":15360,"ancestor_record_offset":15308,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":15408,"parent_record_offset":15384,"ancestor_record_offset":15308,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x186E5
// 首条位置: {"module":2778,"inlinee":100069}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":2778,"record_offset":15200,"parent_record_offset":15168,"ancestor_record_offset":15168,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2778,"record_offset":15360,"parent_record_offset":15336,"ancestor_record_offset":15168,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":15244,"parent_record_offset":15200,"ancestor_record_offset":15168,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":15384,"parent_record_offset":15360,"ancestor_record_offset":15168,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":15408,"parent_record_offset":15384,"ancestor_record_offset":15168,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2778,"record_offset":15336,"parent_record_offset":15308,"ancestor_record_offset":15168,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2778,"record_offset":15280,"parent_record_offset":15200,"ancestor_record_offset":15168,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":2778,"record_offset":15308,"parent_record_offset":15280,"ancestor_record_offset":15168,"inlinee":61910,"source_line":85,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x186E3
// 首条位置: {"module":2778,"inlinee":100067}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2778,"record_offset":15720,"parent_record_offset":14504,"ancestor_record_offset":14504,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2778,"record_offset":17096,"parent_record_offset":14504,"ancestor_record_offset":14504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":15620,"parent_record_offset":14504,"ancestor_record_offset":14504,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":17604,"parent_record_offset":14504,"ancestor_record_offset":14504,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2778,"record_offset":14556,"parent_record_offset":14504,"ancestor_record_offset":14504,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2778,"record_offset":15656,"parent_record_offset":14504,"ancestor_record_offset":14504,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2778,"record_offset":16772,"parent_record_offset":16748,"ancestor_record_offset":14504,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2778,"record_offset":16748,"parent_record_offset":16720,"ancestor_record_offset":14504,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2778,"record_offset":16720,"parent_record_offset":16696,"ancestor_record_offset":14504,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2778,"record_offset":16696,"parent_record_offset":16672,"ancestor_record_offset":14504,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2778,"record_offset":15360,"parent_record_offset":15336,"ancestor_record_offset":14504,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2778,"record_offset":14792,"parent_record_offset":14596,"ancestor_record_offset":14504,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2778,"record_offset":14920,"parent_record_offset":14596,"ancestor_record_offset":14504,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2778,"record_offset":16568,"parent_record_offset":16252,"ancestor_record_offset":14504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":14756,"parent_record_offset":14596,"ancestor_record_offset":14504,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":15384,"parent_record_offset":15360,"ancestor_record_offset":14504,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":15408,"parent_record_offset":15384,"ancestor_record_offset":14504,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2778,"record_offset":15336,"parent_record_offset":15308,"ancestor_record_offset":14504,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":2778,"record_offset":14824,"parent_record_offset":14596,"ancestor_record_offset":14504,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":2778,"record_offset":15200,"parent_record_offset":15168,"ancestor_record_offset":14504,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":2778,"record_offset":14596,"parent_record_offset":14556,"ancestor_record_offset":14504,"inlinee":13307,"source_line":838,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":2778,"record_offset":15784,"parent_record_offset":15748,"ancestor_record_offset":14504,"inlinee":13322,"source_line":857,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::Deserializer::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":2778,"record_offset":16436,"parent_record_offset":16252,"ancestor_record_offset":14504,"inlinee":100071,"source_line":10,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":2778,"record_offset":16004,"parent_record_offset":15980,"ancestor_record_offset":14504,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2778,"record_offset":15280,"parent_record_offset":15200,"ancestor_record_offset":14504,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2778,"record_offset":14864,"parent_record_offset":14596,"ancestor_record_offset":14504,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":2778,"record_offset":15980,"parent_record_offset":15956,"ancestor_record_offset":14504,"inlinee":61913,"source_line":7,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":2778,"record_offset":15308,"parent_record_offset":15280,"ancestor_record_offset":14504,"inlinee":61910,"source_line":85,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":2778,"record_offset":14888,"parent_record_offset":14864,"ancestor_record_offset":14504,"inlinee":100069,"source_line":85,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum<ron::de::Enum>
// 记录形式: compiled_procedure; instances=1
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1186,"record_offset":256,"rva":16438112,"va":5385147232}
// 编译实例 RVAs: 0xfad360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141e3aa00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15259278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385147232,"raw_hex":"e88ed6e800","size":5,"soft_fail":false,"target_va":5400406528,"va":5385147245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e3b460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15261881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385147232,"raw_hex":"e8b9e0e800","size":5,"soft_fail":false,"target_va":5400409184,"va":5385147298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e3cb80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15267707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385147232,"raw_hex":"e87bf7e800","size":5,"soft_fail":false,"target_va":5400415104,"va":5385147392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e3d400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15269840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385147232,"raw_hex":"e8d0ffe800","size":5,"soft_fail":false,"target_va":5400417280,"va":5385147435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1186,"record_offset":556,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1186,"record_offset":464,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1186,"record_offset":500,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1186,"record_offset":436,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1186,"record_offset":528,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5585,"source_line":2117,"archive":"_pdb/B/modules/01186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$21::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF328
// 首条位置: {"module":1211,"inlinee":62248}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tg_text::shape_stream::impl$19::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":1211,"record_offset":15812,"parent_record_offset":15756,"ancestor_record_offset":15756,"inlinee":62250,"source_line":60,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1211,"record_offset":16344,"parent_record_offset":16320,"ancestor_record_offset":15756,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1211,"record_offset":16368,"parent_record_offset":16344,"ancestor_record_offset":15756,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1211,"record_offset":16320,"parent_record_offset":16144,"ancestor_record_offset":15756,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$2::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":1211,"record_offset":15844,"parent_record_offset":15812,"ancestor_record_offset":15756,"inlinee":4241,"source_line":194,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1211,"record_offset":16084,"parent_record_offset":16056,"ancestor_record_offset":15756,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1211,"record_offset":16144,"parent_record_offset":16112,"ancestor_record_offset":15756,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1211,"record_offset":16112,"parent_record_offset":16084,"ancestor_record_offset":15756,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1211,"record_offset":16180,"parent_record_offset":16144,"ancestor_record_offset":15756,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1211,"record_offset":16020,"parent_record_offset":15992,"ancestor_record_offset":15756,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1211,"record_offset":15992,"parent_record_offset":15964,"ancestor_record_offset":15756,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1211,"record_offset":15936,"parent_record_offset":15908,"ancestor_record_offset":15756,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1211,"record_offset":15964,"parent_record_offset":15936,"ancestor_record_offset":15756,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1211,"record_offset":15908,"parent_record_offset":15880,"ancestor_record_offset":15756,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1211,"record_offset":15880,"parent_record_offset":15844,"ancestor_record_offset":15756,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1211,"record_offset":16056,"parent_record_offset":16020,"ancestor_record_offset":15756,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1211,"record_offset":16204,"parent_record_offset":16180,"ancestor_record_offset":15756,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1211,"record_offset":16228,"parent_record_offset":16204,"ancestor_record_offset":15756,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1211,"record_offset":16444,"parent_record_offset":16420,"ancestor_record_offset":15756,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1211,"record_offset":16276,"parent_record_offset":16252,"ancestor_record_offset":15756,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1211,"record_offset":16252,"parent_record_offset":16228,"ancestor_record_offset":15756,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1211,"record_offset":16420,"parent_record_offset":16020,"ancestor_record_offset":15756,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01211.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$22::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x86CB
// 首条位置: {"module":626,"inlinee":34507}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$4::image_name_to_index
// 记录形式: compiled_procedure; instances=1
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":626,"record_offset":2020,"rva":12133120,"va":5380842240}
// 编译实例 RVAs: 0xb92300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30699796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380842240,"raw_hex":"e81471d401","size":5,"soft_fail":false,"target_va":5411542064,"va":5380842263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::RawShapeStream::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x86C9
// 首条位置: {"module":626,"inlinee":34505}
// 编译实例 RVAs: 0xb92320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tg_text::shape_stream::impl$22::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":626,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":1324,"inlinee":34507,"source_line":95,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::RawShapeStream::append
// 记录形式: compiled_procedure; instances=1
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":626,"record_offset":2304,"rva":12133184,"va":5380842304}
// 编译实例 RVAs: 0xb92340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30871348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380842304,"raw_hex":"e8340fd701","size":5,"soft_fail":false,"target_va":5411713920,"va":5380842567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*56(%rsp)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5380842304,"raw_hex":"ff542438","size":4,"soft_fail":false,"va":5380842420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":2452,"parent_record_offset":2304,"ancestor_record_offset":2304,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":626,"record_offset":2944,"parent_record_offset":2304,"ancestor_record_offset":2304,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":626,"record_offset":2536,"parent_record_offset":2304,"ancestor_record_offset":2304,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_direct] tg_text::shape_stream::Image::with_scaling
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":626,"record_offset":2624,"parent_record_offset":2304,"ancestor_record_offset":2304,"inlinee":34510,"source_line":190,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_direct] tg_text::shape_stream::RawShapeStream::encode
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":626,"record_offset":2652,"parent_record_offset":2304,"ancestor_record_offset":2304,"inlinee":34511,"source_line":203,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":3596,"parent_record_offset":3564,"ancestor_record_offset":2304,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":2796,"parent_record_offset":2768,"ancestor_record_offset":2304,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":3508,"parent_record_offset":3480,"ancestor_record_offset":2304,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":2856,"parent_record_offset":2824,"ancestor_record_offset":2304,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":3644,"parent_record_offset":3620,"ancestor_record_offset":2304,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":2304,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":2824,"parent_record_offset":2796,"ancestor_record_offset":2304,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":2736,"parent_record_offset":2708,"ancestor_record_offset":2304,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":3564,"parent_record_offset":3344,"ancestor_record_offset":2304,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":2304,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":2900,"parent_record_offset":2736,"ancestor_record_offset":2304,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":2768,"parent_record_offset":2736,"ancestor_record_offset":2304,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":2680,"parent_record_offset":2652,"ancestor_record_offset":2304,"inlinee":10301,"source_line":3726,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":2304,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":626,"record_offset":2708,"parent_record_offset":2680,"ancestor_record_offset":2304,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":626,"record_offset":2584,"parent_record_offset":2536,"ancestor_record_offset":2304,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::RawShapeStream::clear
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 162, 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x86CC
// 首条位置: {"module":626,"inlinee":34508}
// 编译实例 RVAs: 0xb92590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":1400,"parent_record_offset":1376,"ancestor_record_offset":1376,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::Image::with_scaling
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 190
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x86CE
// 首条位置: {"module":626,"inlinee":34510}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::RawShapeStream::encode
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x86CF
// 首条位置: {"module":626,"inlinee":34511}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$21::extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":2680,"parent_record_offset":2652,"ancestor_record_offset":2652,"inlinee":10301,"source_line":3726,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":3596,"parent_record_offset":3564,"ancestor_record_offset":3260,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":2796,"parent_record_offset":2768,"ancestor_record_offset":2652,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":3508,"parent_record_offset":3480,"ancestor_record_offset":3260,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":2856,"parent_record_offset":2824,"ancestor_record_offset":2652,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":3644,"parent_record_offset":3620,"ancestor_record_offset":3260,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":3260,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":626,"record_offset":2824,"parent_record_offset":2796,"ancestor_record_offset":2652,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":2736,"parent_record_offset":2708,"ancestor_record_offset":2652,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":3564,"parent_record_offset":3344,"ancestor_record_offset":3260,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":2900,"parent_record_offset":2736,"ancestor_record_offset":2652,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":626,"record_offset":2768,"parent_record_offset":2736,"ancestor_record_offset":2652,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00626.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":626,"record_offset":2708,"parent_record_offset":2680,"ancestor_record_offset":2652,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00626.jsonl.gz"}
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
// 完整结构及原始类型记录: shape_stream.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::shape_stream::Image
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::shape_stream::RawShapeStream
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 3
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::shape_stream::ShapeStream
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tg_text::shape_stream::ShapeStream>", "constructor": "core::ptr::drop_in_place", "arguments": ["tg_text::shape_stream::ShapeStream"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_text::shape_stream::ShapeStream::image<u32>", "constructor": "tg_text::shape_stream::ShapeStream::image", "arguments": ["u32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_text::shape_stream::ShapeStream::image<ref$<str$> >", "constructor": "tg_text::shape_stream::ShapeStream::image", "arguments": ["ref$<str$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::shape_stream::ShapeStreamItem
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tg_text::shape_stream::ShapeStreamItem>", "constructor": "enum2$", "arguments": ["tg_text::shape_stream::ShapeStreamItem"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$11::clone<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>", "constructor": "alloc::vec::impl$11::clone", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<enum2$<tg_text::shape_stream::ShapeStreamItem> >", "constructor": "core::marker::PhantomData", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$5::next_element_seed<core::marker::PhantomData<enum2$<tg_text::shape_stream::ShapeStreamItem> > >", "constructor": "ron::de::impl$5::next_element_seed", "arguments": ["core::marker::PhantomData<enum2$<tg_text::shape_stream::ShapeStreamItem> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<enum2$<tg_text::shape_stream::ShapeStreamItem> >", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<enum2$<tg_text::shape_stream::ShapeStreamItem> > >", "constructor": "ron::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<enum2$<tg_text::shape_stream::ShapeStreamItem> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<enum2$<tg_text::shape_stream::ShapeStreamItem>,ron::de::CommaSeparated>", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>", "ron::de::CommaSeparated"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shape_stream.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shape_stream.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile> > (*)(ref_mut$<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >"]

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::Image::with_scaling
// 信息 ID: 5e95ce33f6cf369092820f55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::append; module=626, inline_record=2624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::RawShapeStream::append
// 信息 ID: b2b8565958f8bb6854e1e1fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$9::prepare_layout @ 0x67ecf3 -> 0xb92340 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$12::prepare_layout @ 0x68056f -> 0xb92340 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$25::prepare_layout @ 0x6819f1 -> 0xb92340 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$25::prepare_layout @ 0x681b4f -> 0xb92340 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::shape_stream::ShapeStream::encode_no_images @ 0xb9225f -> 0xb92340 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$2::render @ 0x1dfb866 -> 0xb92340 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$8::prepare_layout @ 0x1e1ed7f -> 0xb92340 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::RawShapeStream::clear
// 信息 ID: 3f70a895ce9f7276bfc42440
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$9::prepare_layout @ 0x67ec99 -> 0xb92590 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$12::prepare_layout @ 0x680559 -> 0xb92590 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$25::prepare_layout @ 0x681997 -> 0xb92590 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$25::prepare_layout @ 0x681a7f -> 0xb92590 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$8::prepare_layout @ 0x1e1ed33 -> 0xb92590 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tg_text::shape_stream::ShapeStream::encode_no_images; module=626, inline_record=1376; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::RawShapeStream::encode
// 信息 ID: 2fb3382a860a60971b62f6fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::append; module=626, inline_record=2652; 不是独立调用边。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::append; module=626, inline_record=2972; 不是独立调用边。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::append; module=626, inline_record=3260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::RawShapeStream::new
// 信息 ID: 15d651b2aed3999485f7bdaf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::credits::impl$2::render @ 0x1dfb830 -> 0xb92320 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tg_text::shape_stream::ShapeStream::encode_no_images; module=626, inline_record=1324; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::corner_radius
// 信息 ID: b01dddc5fe344846938f7ca5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$5::closure$0 @ 0x507da3 -> 0xb92160 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$25::prepare_layout @ 0x6819d7 -> 0xb92160 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x68aa51 -> 0xb92160 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::ui @ 0x1e1bdfd -> 0xb92160 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::encode_no_images
// 信息 ID: a2d0958952ced330f5bceccb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x68aa64 -> 0xb92220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::fill_color
// 信息 ID: c95a7705d2db8c785565cdd6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$5::closure$0 @ 0x507d8f -> 0xb920b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$9::prepare_layout @ 0x67ecdb -> 0xb920b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$25::prepare_layout @ 0x6819c4 -> 0xb920b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$25::prepare_layout @ 0x681b24 -> 0xb920b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x68aa32 -> 0xb920b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$2::render @ 0x1dfb849 -> 0xb920b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::ui @ 0x1e1bddb -> 0xb920b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$8::prepare_layout @ 0x1e1ed69 -> 0xb920b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::image<ref$<str$> >
// 信息 ID: 8747fe900fa5e22651585c86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tg_text::shape_stream::ShapeStream::image -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$25::prepare_layout @ 0x681aa4 -> 0xfad6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$8::prepare_layout @ 0x1e1ed5b -> 0xfad6a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::image<u32>
// 信息 ID: a48b23c6684f3a3b14003e86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_text::shape_stream::ShapeStream::image -> ["u32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$9::prepare_layout @ 0x67ecc5 -> 0xfad550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::is_empty
// 信息 ID: bf5c2957e6f873e2441c28e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$12::render @ 0x68059c -> 0xb92210 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::save_error::ui @ 0x1ec0d1d -> 0xb92210 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::new
// 信息 ID: 81c6857094fad63be5f1b25f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::app_systems::impl$0::add_wall_events @ 0x4940aa -> 0xa82b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$5::closure$0 @ 0x507d7a -> 0xa82b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$9::prepare_layout @ 0x67eca8 -> 0xa82b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$25::prepare_layout @ 0x6819a3 -> 0xa82b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$25::prepare_layout @ 0x681a8b -> 0xa82b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x68aa18 -> 0xa82b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$2::render @ 0x1dfb839 -> 0xa82b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::ui @ 0x1e1bdb3 -> 0xa82b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$8::prepare_layout @ 0x1e1ed3f -> 0xa82b20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize
// 信息 ID: 9be094864710c3470d6de66b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2778, inline_record=15980; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$5::next_element_seed<core::marker::PhantomData<enum2$<tg_text::shape_stream::ShapeStreamItem> > >; module=2778, inline_record=33140; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<toolbar::mei::config::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=79960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: dea99399aebd51935f08f105
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::shape_stream::_::impl$0::deserialize::__Field> >; module=2778, inline_record=11164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 919a177f54901eb867281bd8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::shape_stream::_::impl$0::deserialize::__Field> > @ 0x1e3a8f6 -> 0xfad490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 937e4932f095eabce979ced6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::shape_stream::_::impl$0::deserialize::__Field> >; module=2778, inline_record=10440; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::shape_stream::_::impl$0::deserialize::__Field> >; module=2778, inline_record=11020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 8c140a31492314aa0d39212c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<tg_text::shape_stream::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=63820; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: 22b52e286eac28f7eb5e417f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2778, inline_record=15308; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__FieldVisitor>; module=3942, inline_record=7400; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__FieldVisitor>; module=3942, inline_record=7824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: 68b58203acba15536b13964d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2778, inline_record=14888; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2778, inline_record=15044; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2778, inline_record=15168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: 1d52b0aa4b074b5683a365bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2778, inline_record=14504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum<ron::de::Enum>
// 信息 ID: 01b32037ae9a3dfb019bd9ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum -> ["ron::de::Enum"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_enum<tg_text::shape_stream::_::impl$0::deserialize::__Visitor> @ 0x1e44633 -> 0xfad360 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize<ref_mut$<ron::de::tag::Deserializer> >
// 信息 ID: bc701622d3f478cb46434021
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::tag::Deserializer"]; 字段布局 = unknown。
// [类型关系] tg_text::shape_stream::_::impl$0::deserialize -> ["ref_mut$<ron::de::tag::Deserializer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1e3bc90 -> 0xfad7f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$19::clone
// 信息 ID: 043eea6642c4597b0a54e9dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$11::clone<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>; module=1211, inline_record=15812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$1::from
// 信息 ID: 74617f8bcd929ca90a6c3a2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_text::shape_stream::ShapeStream::image<u32> @ 0xfad578 -> 0xb922c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$21::clone
// 信息 ID: bf5863eaaf0162e09d5b520c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$11::clone<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>; module=1211, inline_record=15756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$22::default
// 信息 ID: cb295f0932d481ada1b15d78
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_text::shape_stream::ShapeStream::encode_no_images; module=626, inline_record=1348; 不是独立调用边。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::new; module=626, inline_record=2276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$2::from
// 信息 ID: d2bcc703ec02656a89748348
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_text::shape_stream::ShapeStream::image<ref$<str$> > @ 0xfad6c6 -> 0xb922e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$3::from
// 信息 ID: fbd9c22c83207f1eea47f965
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 38 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings @ 0x245fb5 -> 0x681f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings @ 0x2462c5 -> 0x681f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_video @ 0x248376 -> 0x681f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_video @ 0x2492a5 -> 0x681f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_developer @ 0x24b100 -> 0x681f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x32ecc8 -> 0x681f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x32ef50 -> 0x681f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x32f353 -> 0x681f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x32f574 -> 0x681f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode> > @ 0x32f903 -> 0x681f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode> > @ 0x32fb24 -> 0x681f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<country_core::render::render_config::ImageQualityPreset> > @ 0x32feb3 -> 0x681f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<country_core::render::render_config::ImageQualityPreset> > @ 0x3300a5 -> 0x681f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x3304a3 -> 0x681f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x3306c4 -> 0x681f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x330a55 -> 0x681f40 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$4::image_name_to_index
// 信息 ID: 2cd908192ab862342a4de55e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$8::clone
// 信息 ID: 82180f356b9038b5bca116bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=342, inline_record=2016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
