// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tg-iced/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 0c2c7daa459f91b0df62c2ef34918d29816d0d9bef71b63f9b4c74abe9494613
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 35, 184, 272, 931, 945, 975, 981, 986, 989, 990, 993, 994, 1001, 2376, 2380, 2389, 2498, 2507, 3825, 3827, 4992
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.0"}]
// use iced_core; // namespace candidate
//   package identities: [{"name":"iced_core","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_graphics; // namespace candidate
//   package identities: [{"name":"iced_graphics","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_tiny_skia; // namespace candidate
//   package identities: [{"name":"iced_tiny_skia","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use std; // namespace candidate
//   package identities: []
// use strict_num; // namespace candidate
//   package identities: [{"name":"strict-num","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.1.1"}]
// use tiny_skia; // namespace candidate
//   package identities: [{"name":"tiny-skia","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.4"}]
// use tiny_skia_path; // namespace candidate
//   package identities: [{"name":"tiny-skia-path","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.4"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: iced_graphics::damage::impl$0::bounds<enum2$<iced_tiny_skia::primitive::Custom> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3827,"record_offset":11940,"rva":42209104,"va":5410918224}
// 编译实例 RVAs: 0x2840f50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142840f50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410918224,"raw_hex":"e8f7feffff","size":5,"soft_fail":false,"target_va":5410918224,"va":5410918484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::transformation::_$LT$impl$u20$core..ops..arith..Mul$LT$iced_core..transformation..Transformation$GT$$u20$for$u20$iced_core..rectangle..Rectangle$GT$::mul
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-34967677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410918224,"raw_hex":"e8836feafd","size":5,"soft_fail":false,"target_va":5375950864,"va":5410918536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3827,"record_offset":12132,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3827,"record_offset":12368,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3827,"record_offset":12244,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3827,"record_offset":12216,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3827,"record_offset":12424,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_direct] iced_core::rectangle::Rectangle::expand
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3827,"record_offset":12272,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":5474,"source_line":152,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_direct] iced_tiny_skia::primitive::impl$0::bounds
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/primitive.rs
//     evidence={"module":3827,"record_offset":12732,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":16594,"source_line":29,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3827,"record_offset":12156,"parent_record_offset":12132,"ancestor_record_offset":11940,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3827,"record_offset":12392,"parent_record_offset":12368,"ancestor_record_offset":11940,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3827,"record_offset":12684,"parent_record_offset":12612,"ancestor_record_offset":11940,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3827,"record_offset":12652,"parent_record_offset":12612,"ancestor_record_offset":11940,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3827,"record_offset":12532,"parent_record_offset":12452,"ancestor_record_offset":11940,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3827,"record_offset":12556,"parent_record_offset":12532,"ancestor_record_offset":11940,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3827,"record_offset":12180,"parent_record_offset":12156,"ancestor_record_offset":11940,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3827,"record_offset":12504,"parent_record_offset":12452,"ancestor_record_offset":11940,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3827,"record_offset":12452,"parent_record_offset":12424,"ancestor_record_offset":11940,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::rectangle::Rectangle::expand
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3827,"record_offset":12820,"parent_record_offset":12732,"ancestor_record_offset":11940,"inlinee":5474,"source_line":152,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::rectangle::Rectangle::union
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3827,"record_offset":12612,"parent_record_offset":12584,"ancestor_record_offset":11940,"inlinee":16592,"source_line":123,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_transitive] iced_graphics::damage::impl$0::bounds::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/graphics/src/damage.rs
//     evidence={"module":3827,"record_offset":12584,"parent_record_offset":12532,"ancestor_record_offset":11940,"inlinee":16591,"source_line":102,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::path::Path::bounds
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/path.rs
//     evidence={"module":3827,"record_offset":12764,"parent_record_offset":12732,"ancestor_record_offset":11940,"inlinee":16596,"source_line":69,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::rect::Rect::width
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/rect.rs
//     evidence={"module":3827,"record_offset":12792,"parent_record_offset":12732,"ancestor_record_offset":11940,"inlinee":16598,"source_line":294,"archive":"_pdb/C/modules/03827.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3825,"record_offset":264,"rva":42197824,"va":5410906944}
// 编译实例 RVAs: 0x283e340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::components
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-26257228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e8b4586ffe","size":5,"soft_fail":false,"target_va":5384653152,"va":5410910375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9257425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e82fbe72ff","size":5,"soft_fail":false,"target_va":5401649760,"va":5410907180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_core..image..Bytes$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-34959136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e8e090eafd","size":5,"soft_fail":false,"target_va":5375950304,"va":5410909435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_graphics..text..editor..Weak$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-34968035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e81d6eeafd","size":5,"soft_fail":false,"target_va":5375940768,"va":5410908798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_graphics..text..paragraph..Weak$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-34965344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e8a078eafd","size":5,"soft_fail":false,"target_va":5375942880,"va":5410908219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-35229316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e87c71e6fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5410907999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42185797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e8bb4b7cfd","size":5,"soft_fail":false,"target_va":5368722192,"va":5410907984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fa670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-39075731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e86dc0abfd","size":5,"soft_fail":false,"target_va":5371831920,"va":5410907646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fa790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-39075182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e892c2abfd","size":5,"soft_fail":false,"target_va":5371832208,"va":5410907385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405a9550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36264064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e880a7d6fd","size":5,"soft_fail":false,"target_va":5374645584,"va":5410909643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14283e340
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e890feffff","size":5,"soft_fail":false,"target_va":5410906944,"va":5410907307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428402e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e8d3140000","size":5,"soft_fail":false,"target_va":5410915040,"va":5410909704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142840f50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e8671c0000","size":5,"soft_fail":false,"target_va":5410918224,"va":5410910948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6148647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e827d25d00","size":5,"soft_fail":false,"target_va":5417057040,"va":5410908388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5709571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e8031f5700","size":5,"soft_fail":false,"target_va":5416620769,"va":5410911193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::iter::traits::iterator::Iterator::eq_by
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2810685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e83de32a00","size":5,"soft_fail":false,"target_va":5413721328,"va":5410910638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::border::impl$12::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-36536721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e86f7ed2fd","size":5,"soft_fail":false,"target_va":5374374160,"va":5410910876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::font::impl$6::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-36534602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e8b686d2fd","size":5,"soft_fail":false,"target_va":5374374048,"va":5410908645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::shadow::impl$5::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-36536625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e8cf7ed2fd","size":5,"soft_fail":false,"target_va":5374374288,"va":5410910908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::svg::impl$12::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t1426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e892050000","size":5,"soft_fail":false,"target_va":5410911216,"va":5410909785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_skia::shaders::impl$10::eq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e81b090000","size":5,"soft_fail":false,"target_va":5410911616,"va":5410909280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_skia_path::stroker::impl$11::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-36534055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410906944,"raw_hex":"e8d988d2fd","size":5,"soft_fail":false,"target_va":5374375280,"va":5410909330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3825,"record_offset":9420,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3825,"record_offset":9236,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::ptr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3825,"record_offset":2160,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7951,"source_line":2143,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3825,"record_offset":2216,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3825,"record_offset":1932,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3825,"record_offset":468,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3825,"record_offset":1492,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3825,"record_offset":2104,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3825,"record_offset":2640,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":16011,"source_line":957,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3825,"record_offset":2048,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_direct] iced_core::rectangle::impl$9::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3825,"record_offset":9504,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":16010,"source_line":4,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_direct] iced_graphics::damage::list
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/graphics/src/damage.rs
//     evidence={"module":3825,"record_offset":776,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":16004,"source_line":186,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3825,"record_offset":9320,"parent_record_offset":9296,"ancestor_record_offset":264,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3825,"record_offset":9344,"parent_record_offset":9320,"ancestor_record_offset":264,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3825,"record_offset":9368,"parent_record_offset":9344,"ancestor_record_offset":264,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3825,"record_offset":9296,"parent_record_offset":9272,"ancestor_record_offset":264,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3825,"record_offset":2312,"parent_record_offset":2216,"ancestor_record_offset":264,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$18::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3825,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":264,"inlinee":16021,"source_line":2086,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3825,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":264,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3825,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3825,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":264,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3825,"record_offset":2256,"parent_record_offset":2216,"ancestor_record_offset":264,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3825,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":264,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3825,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":264,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::ptr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3825,"record_offset":7972,"parent_record_offset":7948,"ancestor_record_offset":264,"inlinee":7951,"source_line":2143,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3825,"record_offset":3076,"parent_record_offset":3052,"ancestor_record_offset":264,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$53::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3825,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":264,"inlinee":16019,"source_line":3506,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$54::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3825,"record_offset":7948,"parent_record_offset":7924,"ancestor_record_offset":264,"inlinee":16065,"source_line":3525,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$55::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3825,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":264,"inlinee":16017,"source_line":3555,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3825,"record_offset":516,"parent_record_offset":492,"ancestor_record_offset":264,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3825,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":264,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3825,"record_offset":2280,"parent_record_offset":2256,"ancestor_record_offset":264,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3825,"record_offset":4228,"parent_record_offset":4204,"ancestor_record_offset":264,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3825,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":264,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3825,"record_offset":4276,"parent_record_offset":4228,"ancestor_record_offset":264,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":3825,"record_offset":4204,"parent_record_offset":4180,"ancestor_record_offset":264,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":3825,"record_offset":8416,"parent_record_offset":8364,"ancestor_record_offset":264,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$8::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":3825,"record_offset":8440,"parent_record_offset":8416,"ancestor_record_offset":264,"inlinee":16063,"source_line":143,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3825,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":264,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3825,"record_offset":5876,"parent_record_offset":5840,"ancestor_record_offset":264,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3825,"record_offset":7768,"parent_record_offset":7744,"ancestor_record_offset":264,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3825,"record_offset":2404,"parent_record_offset":2368,"ancestor_record_offset":264,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3825,"record_offset":3004,"parent_record_offset":2800,"ancestor_record_offset":264,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3825,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":264,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmpeq_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3825,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":264,"inlinee":11124,"source_line":503,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3825,"record_offset":1400,"parent_record_offset":1376,"ancestor_record_offset":264,"inlinee":8137,"source_line":36,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3825,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":264,"inlinee":16013,"source_line":23,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3825,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":264,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":3825,"record_offset":7672,"parent_record_offset":7640,"ancestor_record_offset":264,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":3825,"record_offset":1056,"parent_record_offset":920,"ancestor_record_offset":264,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":3825,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":264,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3825,"record_offset":1376,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":8135,"source_line":507,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3825,"record_offset":1172,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3825,"record_offset":7640,"parent_record_offset":7404,"ancestor_record_offset":264,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3825,"record_offset":920,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":3825,"record_offset":1960,"parent_record_offset":1932,"ancestor_record_offset":264,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":3825,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":264,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3825,"record_offset":7496,"parent_record_offset":7472,"ancestor_record_offset":264,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3825,"record_offset":8824,"parent_record_offset":8800,"ancestor_record_offset":264,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::addr_eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3825,"record_offset":2184,"parent_record_offset":2160,"ancestor_record_offset":264,"inlinee":7952,"source_line":2443,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3825,"record_offset":884,"parent_record_offset":860,"ancestor_record_offset":264,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3825,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":264,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3825,"record_offset":7580,"parent_record_offset":7556,"ancestor_record_offset":264,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal_same_length
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3825,"record_offset":5840,"parent_record_offset":5748,"ancestor_record_offset":264,"inlinee":6223,"source_line":122,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3825,"record_offset":4620,"parent_record_offset":4228,"ancestor_record_offset":264,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3825,"record_offset":836,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3825,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":264,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3825,"record_offset":1228,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3825,"record_offset":1260,"parent_record_offset":1228,"ancestor_record_offset":264,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3825,"record_offset":860,"parent_record_offset":836,"ancestor_record_offset":264,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3825,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":264,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::impl$17::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.25.0/src/f32/sse2/mat4.rs
//     evidence={"module":3825,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":264,"inlinee":16008,"source_line":1376,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::cmpeq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.25.0/src/f32/sse2/vec4.rs
//     evidence={"module":3825,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":264,"inlinee":11123,"source_line":250,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$2::eq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.25.0/src/f32/sse2/vec4.rs
//     evidence={"module":3825,"record_offset":1576,"parent_record_offset":1540,"ancestor_record_offset":264,"inlinee":11121,"source_line":803,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::alignment::impl$15::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/alignment.rs
//     evidence={"module":3825,"record_offset":5064,"parent_record_offset":5040,"ancestor_record_offset":264,"inlinee":16031,"source_line":37,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::alignment::impl$23::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/alignment.rs
//     evidence={"module":3825,"record_offset":5120,"parent_record_offset":5096,"ancestor_record_offset":264,"inlinee":16033,"source_line":50,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::angle::impl$27::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/angle.rs
//     evidence={"module":3825,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":264,"inlinee":16061,"source_line":11,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::background::impl$8::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/background.rs
//     evidence={"module":3825,"record_offset":8260,"parent_record_offset":8232,"ancestor_record_offset":264,"inlinee":16055,"source_line":5,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::color::impl$12::eq
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/color.rs
//     evidence={"module":3825,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":264,"inlinee":16025,"source_line":5,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::gradient::impl$21::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/gradient.rs
//     evidence={"module":3825,"record_offset":8364,"parent_record_offset":8340,"ancestor_record_offset":264,"inlinee":16059,"source_line":50,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::gradient::impl$8::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/gradient.rs
//     evidence={"module":3825,"record_offset":8316,"parent_record_offset":8292,"ancestor_record_offset":264,"inlinee":16057,"source_line":6,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$14::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":3825,"record_offset":7252,"parent_record_offset":7216,"ancestor_record_offset":264,"inlinee":16049,"source_line":9,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$19::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":3825,"record_offset":7284,"parent_record_offset":7252,"ancestor_record_offset":264,"inlinee":16051,"source_line":138,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$27::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":3825,"record_offset":9136,"parent_record_offset":9108,"ancestor_record_offset":264,"inlinee":16080,"source_line":170,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::pixels::impl$9::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/pixels.rs
//     evidence={"module":3825,"record_offset":4924,"parent_record_offset":4900,"ancestor_record_offset":264,"inlinee":16027,"source_line":9,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::point::impl$13::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/point.rs
//     evidence={"module":3825,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":264,"inlinee":16023,"source_line":7,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::rectangle::impl$9::eq
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3825,"record_offset":2368,"parent_record_offset":2344,"ancestor_record_offset":264,"inlinee":16010,"source_line":4,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::svg::impl$7::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/svg.rs
//     evidence={"module":3825,"record_offset":7896,"parent_record_offset":7868,"ancestor_record_offset":264,"inlinee":16053,"source_line":10,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::text::impl$14::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/text.rs
//     evidence={"module":3825,"record_offset":5176,"parent_record_offset":5152,"ancestor_record_offset":264,"inlinee":16035,"source_line":46,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::text::impl$23::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/text.rs
//     evidence={"module":3825,"record_offset":4980,"parent_record_offset":4956,"ancestor_record_offset":264,"inlinee":16029,"source_line":70,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::transformation::impl$14::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/transformation.rs
//     evidence={"module":3825,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":264,"inlinee":16006,"source_line":7,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_graphics::primitive::impl$4::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/graphics/src/primitive.rs
//     evidence={"module":3825,"record_offset":2800,"parent_record_offset":2752,"ancestor_record_offset":264,"inlinee":16015,"source_line":16,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::primitive::impl$4::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/primitive.rs
//     evidence={"module":3825,"record_offset":5616,"parent_record_offset":5584,"ancestor_record_offset":264,"inlinee":16037,"source_line":6,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":3825,"record_offset":7472,"parent_record_offset":7404,"ancestor_record_offset":264,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":3825,"record_offset":7444,"parent_record_offset":7404,"ancestor_record_offset":264,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":3825,"record_offset":7404,"parent_record_offset":7380,"ancestor_record_offset":264,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":3825,"record_offset":7380,"parent_record_offset":7356,"ancestor_record_offset":264,"inlinee":16069,"source_line":2218,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] strict_num::impl$2::eq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"strict-num","version":"0.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/strict-num-0.1.1/src/lib.rs
//     evidence={"module":3825,"record_offset":6188,"parent_record_offset":6164,"ancestor_record_offset":264,"inlinee":8346,"source_line":137,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia::blend_mode::impl$7::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/blend_mode.rs
//     evidence={"module":3825,"record_offset":6360,"parent_record_offset":6336,"ancestor_record_offset":264,"inlinee":16047,"source_line":4,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia::painter::impl$11::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/painter.rs
//     evidence={"module":3825,"record_offset":7176,"parent_record_offset":7152,"ancestor_record_offset":264,"inlinee":16067,"source_line":22,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia::painter::impl$15::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/painter.rs
//     evidence={"module":3825,"record_offset":6336,"parent_record_offset":6312,"ancestor_record_offset":264,"inlinee":16045,"source_line":37,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::impl$13::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/lib.rs
//     evidence={"module":3825,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":264,"inlinee":16041,"source_line":70,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::path::impl$15::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/path.rs
//     evidence={"module":3825,"record_offset":5692,"parent_record_offset":5668,"ancestor_record_offset":264,"inlinee":16039,"source_line":48,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::path::impl$9::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/path.rs
//     evidence={"module":3825,"record_offset":5900,"parent_record_offset":5876,"ancestor_record_offset":264,"inlinee":9228,"source_line":18,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::rect::impl$15::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/rect.rs
//     evidence={"module":3825,"record_offset":6164,"parent_record_offset":5692,"ancestor_record_offset":264,"inlinee":16043,"source_line":214,"archive":"_pdb/C/modules/03825.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::text
// 记录形式: inline_instance, inlinee_declaration; instances=66
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x49FB
// 首条位置: {"module":272,"inlinee":18939}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":1184,"parent_record_offset":1140,"ancestor_record_offset":1140,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":1880,"parent_record_offset":1140,"ancestor_record_offset":1140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":272,"record_offset":10256,"parent_record_offset":9568,"ancestor_record_offset":9568,"inlinee":18708,"source_line":113,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":1140,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":1140,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=53
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":1140,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=53
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":1140,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":1736,"parent_record_offset":1712,"ancestor_record_offset":1140,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=53
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":1140,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":1664,"parent_record_offset":1508,"ancestor_record_offset":1140,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=53
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":2056,"parent_record_offset":1992,"ancestor_record_offset":1140,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":1444,"parent_record_offset":1416,"ancestor_record_offset":1140,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=53
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":2024,"parent_record_offset":1992,"ancestor_record_offset":1140,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=53
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":1992,"parent_record_offset":1964,"ancestor_record_offset":1140,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":1508,"parent_record_offset":1472,"ancestor_record_offset":1140,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":1472,"parent_record_offset":1444,"ancestor_record_offset":1140,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=53
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":1964,"parent_record_offset":1936,"ancestor_record_offset":1140,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":1552,"parent_record_offset":1508,"ancestor_record_offset":1140,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":1324,"parent_record_offset":1296,"ancestor_record_offset":1140,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":1352,"parent_record_offset":1324,"ancestor_record_offset":1140,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":1380,"parent_record_offset":1352,"ancestor_record_offset":1140,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":1296,"parent_record_offset":1268,"ancestor_record_offset":1140,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":272,"record_offset":1268,"parent_record_offset":1240,"ancestor_record_offset":1140,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":34256,"parent_record_offset":34204,"ancestor_record_offset":34132,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$114::spec_to_string
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":1212,"parent_record_offset":1184,"ancestor_record_offset":1140,"inlinee":10264,"source_line":3043,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$125::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":24280,"parent_record_offset":24252,"ancestor_record_offset":24216,"inlinee":18944,"source_line":3043,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$126::spec_to_string
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":23088,"parent_record_offset":23060,"ancestor_record_offset":23032,"inlinee":6120,"source_line":3043,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$28::fmt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":34336,"parent_record_offset":34312,"ancestor_record_offset":34132,"inlinee":4606,"source_line":2711,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":34204,"parent_record_offset":34168,"ancestor_record_offset":34132,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":1240,"parent_record_offset":1212,"ancestor_record_offset":1140,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":1416,"parent_record_offset":1380,"ancestor_record_offset":1140,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":272,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":1140,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":986,"record_offset":34284,"parent_record_offset":34204,"ancestor_record_offset":34132,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$82::fmt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":986,"record_offset":34312,"parent_record_offset":34204,"ancestor_record_offset":34132,"inlinee":6317,"source_line":2865,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":272,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":1140,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":272,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":1140,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":272,"record_offset":1792,"parent_record_offset":1380,"ancestor_record_offset":1140,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":1140,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=109
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":1908,"parent_record_offset":1880,"ancestor_record_offset":1140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":986,"record_offset":34368,"parent_record_offset":34204,"ancestor_record_offset":34132,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::impl$0::default
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3CF2
// 首条位置: {"module":184,"inlinee":15602}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_iced::ProgramWithOutcome
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 10; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)
// 候选参数类型: ["ref$<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: iced_graphics::damage::impl$0::bounds<enum2$<iced_tiny_skia::primitive::Custom> >
// 信息 ID: 15f53d325275b6ef70945805
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 089084bcf65bc8c4f0175768: iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// [类型关系] enum2$ -> ["iced_tiny_skia::primitive::Custom"]; 字段布局 = unknown。
// [类型关系] iced_graphics::damage::impl$0::bounds -> ["enum2$<iced_tiny_skia::primitive::Custom>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::chain::Chain<core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)> > > @ 0x1f6a9f1 -> 0x2840f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::chain::Chain<core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)> > > @ 0x1f6a7b5 -> 0x2840f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> > @ 0x283f2e4 -> 0x2840f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> > @ 0x283f2f1 -> 0x2840f50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> >
// 信息 ID: b3a2f2271618f3c24ef65d87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["iced_tiny_skia::primitive::Custom"]; 字段布局 = unknown。
// [类型关系] iced_graphics::damage::regions -> ["enum2$<iced_tiny_skia::primitive::Custom>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > > @ 0x1f68c3d -> 0x283e340 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > > @ 0x1f68d46 -> 0x283e340 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::impl$0::default
// 信息 ID: 6a1f07061d37ba1bfd673894
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=184, inline_record=31156; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_integrity_report::controls::Controls>; module=945, inline_record=6088; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_gpu_selector::vulkan_config::Controls>; module=990, inline_record=6084; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_iced::dialog::Controls>; module=1001, inline_record=6040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::text
// 信息 ID: 05cdfb6a0dc6042026c7a3c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 18d8741a28d27deaef6be6a5: iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 54d9f831789c338436a69108: iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 80d49587b3c5a95afabc153a: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 82810ea9d2375ca9b2c82431: iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] dc12d99c00e7307ec4b5db0b: iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] e98aff1265803b4c26f26dec: iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 0 条；内联宿主 54 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_gpu_selector::gpu_selection::render_error_to_ui; module=272, inline_record=1140; 不是独立调用边。
// [PDB 内联宿主] tg_gpu_selector::gpu_selection::render_error_to_ui; module=272, inline_record=3188; 不是独立调用边。
// [PDB 内联宿主] tg_gpu_selector::gpu_selection::render_error_to_ui; module=272, inline_record=5424; 不是独立调用边。
// [PDB 内联宿主] tg_gpu_selector::gpu_selection::device_driver_advice; module=272, inline_record=9568; 不是独立调用边。
// [PDB 内联宿主] tg_gpu_selector::gpu_selection::device_driver_advice; module=272, inline_record=12112; 不是独立调用边。
// [PDB 内联宿主] tg_gpu_selector::gpu_selection::device_driver_advice; module=272, inline_record=16020; 不是独立调用边。
// [PDB 内联宿主] tg_gpu_selector::gpu_selection::device_driver_advice; module=272, inline_record=19000; 不是独立调用边。
// [PDB 内联宿主] tg_gpu_selector::gpu_selection::device_driver_advice; module=272, inline_record=21348; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
