// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tg-iced/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a196b255f88d0f6bb3409842d095e3db2b267d7d833f22c6dde451a79f3d04f9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 882, 938, 945, 2201, 2206, 2319, 2322, 2338, 3503, 3507, 3525, 3529
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
// 首条位置: {"module":3529,"record_offset":11628,"rva":35476432,"va":5404185552}
// 编译实例 RVAs: 0x21d53d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1421d53d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404185552,"raw_hex":"e8f9feffff","size":5,"soft_fail":false,"target_va":5404185552,"va":5404185810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::rectangle::Rectangle::union
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-28857974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404185552,"raw_hex":"e88aa947fe","size":5,"soft_fail":false,"target_va":5375328016,"va":5404185985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::transformation::_$LT$impl$u20$core..ops..arith..Mul$LT$iced_core..transformation..Transformation$GT$$u20$for$u20$iced_core..rectangle..Rectangle$GT$::mul
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-28857643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404185552,"raw_hex":"e8d5aa47fe","size":5,"soft_fail":false,"target_va":5375328224,"va":5404185862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3529,"record_offset":11820,"parent_record_offset":11628,"ancestor_record_offset":11628,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3529,"record_offset":12056,"parent_record_offset":11628,"ancestor_record_offset":11628,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3529,"record_offset":11936,"parent_record_offset":11628,"ancestor_record_offset":11628,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3529,"record_offset":11904,"parent_record_offset":11628,"ancestor_record_offset":11628,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3529,"record_offset":12112,"parent_record_offset":11628,"ancestor_record_offset":11628,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] iced_core::rectangle::Rectangle::expand
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3529,"record_offset":11964,"parent_record_offset":11628,"ancestor_record_offset":11628,"inlinee":10512,"source_line":152,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] iced_tiny_skia::primitive::impl$0::bounds
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/primitive.rs
//     evidence={"module":3529,"record_offset":12312,"parent_record_offset":11628,"ancestor_record_offset":11628,"inlinee":10516,"source_line":29,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3529,"record_offset":11844,"parent_record_offset":11820,"ancestor_record_offset":11628,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3529,"record_offset":12080,"parent_record_offset":12056,"ancestor_record_offset":11628,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3529,"record_offset":12220,"parent_record_offset":12140,"ancestor_record_offset":11628,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3529,"record_offset":12244,"parent_record_offset":12220,"ancestor_record_offset":11628,"inlinee":4196,"source_line":166,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3529,"record_offset":11868,"parent_record_offset":11844,"ancestor_record_offset":11628,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3529,"record_offset":12192,"parent_record_offset":12140,"ancestor_record_offset":11628,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3529,"record_offset":12140,"parent_record_offset":12112,"ancestor_record_offset":11628,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::rectangle::Rectangle::expand
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3529,"record_offset":12400,"parent_record_offset":12312,"ancestor_record_offset":11628,"inlinee":10512,"source_line":152,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] iced_graphics::damage::impl$0::bounds::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/graphics/src/damage.rs
//     evidence={"module":3529,"record_offset":12272,"parent_record_offset":12220,"ancestor_record_offset":11628,"inlinee":10514,"source_line":102,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::path::Path::bounds
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/path.rs
//     evidence={"module":3529,"record_offset":12344,"parent_record_offset":12312,"ancestor_record_offset":11628,"inlinee":10518,"source_line":69,"archive":"_pdb/B/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::rect::Rect::width
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/rect.rs
//     evidence={"module":3529,"record_offset":12372,"parent_record_offset":12312,"ancestor_record_offset":11628,"inlinee":10520,"source_line":294,"archive":"_pdb/B/modules/03529.jsonl.gz"}
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
// 首条位置: {"module":3525,"record_offset":256,"rva":35465328,"va":5404174448}
// 编译实例 RVAs: 0x21d2870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7730288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e8900b8aff","size":5,"soft_fail":false,"target_va":5396444400,"va":5404174683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_core..image..Bytes$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28851639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e849c247fe","size":5,"soft_fail":false,"target_va":5375325088,"va":5404176722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_graphics..text..editor..Weak$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28862887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e8599647fe","size":5,"soft_fail":false,"target_va":5375313472,"va":5404176354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_graphics..text..paragraph..Weak$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28853364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e88cbb47fe","size":5,"soft_fail":false,"target_va":5375322416,"va":5404175775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b6580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32622016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e8403a0efe","size":5,"soft_fail":false,"target_va":5371553152,"va":5404175163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b67f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32621124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e8bc3d0efe","size":5,"soft_fail":false,"target_va":5371553776,"va":5404174895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140510730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30157553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e80fd533fe","size":5,"soft_fail":false,"target_va":5374019376,"va":5404176924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29094054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e85a0f44fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5404175537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421d2870
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e891feffff","size":5,"soft_fail":false,"target_va":5404174448,"va":5404174810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421d4850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e8e4150000","size":5,"soft_fail":false,"target_va":5404182608,"va":5404176999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421d53d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e8a41c0000","size":5,"soft_fail":false,"target_va":5404185552,"va":5404178215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7671203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e8a30d7500","size":5,"soft_fail":false,"target_va":5411847152,"va":5404175944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7362526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e8de577000","size":5,"soft_fail":false,"target_va":5411540992,"va":5404178461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::border::impl$11::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-33195213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e8337b05fe","size":5,"soft_fail":false,"target_va":5370982576,"va":5404177784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::font::impl$5::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-33193742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e8f28005fe","size":5,"soft_fail":false,"target_va":5370982464,"va":5404176201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::shadow::impl$4::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-33195121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e88f7b05fe","size":5,"soft_fail":false,"target_va":5370982704,"va":5404177820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::svg::impl$12::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t1956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e8a4070000","size":5,"soft_fail":false,"target_va":5404179024,"va":5404177063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e877040000","size":5,"soft_fail":false,"target_va":5404178496,"va":5404177348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_skia::shaders::impl$9::eq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e89e040000","size":5,"soft_fail":false,"target_va":5404179136,"va":5404177949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_skia_path::stroker::impl$11::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-33194772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404174448,"raw_hex":"e8ec7c05fe","size":5,"soft_fail":false,"target_va":5370983264,"va":5404178031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3525,"record_offset":8800,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3525,"record_offset":8956,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::ptr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3525,"record_offset":2204,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":8149,"source_line":1933,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3525,"record_offset":2260,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3525,"record_offset":2004,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3525,"record_offset":460,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3525,"record_offset":1564,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3525,"record_offset":2148,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3525,"record_offset":2632,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":15461,"source_line":911,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3525,"record_offset":2092,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_direct] iced_core::rectangle::impl$9::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3525,"record_offset":9040,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":10454,"source_line":4,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_direct] iced_graphics::damage::list
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/graphics/src/damage.rs
//     evidence={"module":3525,"record_offset":768,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":14003,"source_line":186,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3525,"record_offset":8860,"parent_record_offset":8836,"ancestor_record_offset":256,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3525,"record_offset":8884,"parent_record_offset":8860,"ancestor_record_offset":256,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3525,"record_offset":8836,"parent_record_offset":8800,"ancestor_record_offset":256,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$15::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3525,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":256,"inlinee":10456,"source_line":1822,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3525,"record_offset":532,"parent_record_offset":508,"ancestor_record_offset":256,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3525,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":256,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3525,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":256,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3525,"record_offset":2304,"parent_record_offset":2260,"ancestor_record_offset":256,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3525,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":256,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3525,"record_offset":3096,"parent_record_offset":3072,"ancestor_record_offset":256,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::ptr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3525,"record_offset":8292,"parent_record_offset":8268,"ancestor_record_offset":256,"inlinee":8149,"source_line":1933,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3525,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":256,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$47::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3525,"record_offset":3048,"parent_record_offset":3024,"ancestor_record_offset":256,"inlinee":10452,"source_line":3246,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$48::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3525,"record_offset":8268,"parent_record_offset":8244,"ancestor_record_offset":256,"inlinee":10498,"source_line":3265,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$49::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3525,"record_offset":3024,"parent_record_offset":3000,"ancestor_record_offset":256,"inlinee":10450,"source_line":3295,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3525,"record_offset":508,"parent_record_offset":484,"ancestor_record_offset":256,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3525,"record_offset":484,"parent_record_offset":460,"ancestor_record_offset":256,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3525,"record_offset":2328,"parent_record_offset":2304,"ancestor_record_offset":256,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3525,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":256,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":3525,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":256,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":3525,"record_offset":7792,"parent_record_offset":7740,"ancestor_record_offset":256,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$8::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":3525,"record_offset":7816,"parent_record_offset":7792,"ancestor_record_offset":256,"inlinee":10417,"source_line":137,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3525,"record_offset":1116,"parent_record_offset":1092,"ancestor_record_offset":256,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3525,"record_offset":5656,"parent_record_offset":5472,"ancestor_record_offset":256,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3525,"record_offset":7328,"parent_record_offset":7304,"ancestor_record_offset":256,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3525,"record_offset":2424,"parent_record_offset":2388,"ancestor_record_offset":256,"inlinee":8507,"source_line":1740,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3525,"record_offset":5564,"parent_record_offset":5536,"ancestor_record_offset":256,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=45
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3525,"record_offset":3000,"parent_record_offset":2796,"ancestor_record_offset":256,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3525,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":256,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmpeq_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3525,"record_offset":1696,"parent_record_offset":1672,"ancestor_record_offset":256,"inlinee":10466,"source_line":483,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3525,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":256,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3525,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":256,"inlinee":15463,"source_line":23,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3525,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":256,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3525,"record_offset":1044,"parent_record_offset":908,"ancestor_record_offset":256,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3525,"record_offset":1068,"parent_record_offset":1044,"ancestor_record_offset":256,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3525,"record_offset":5592,"parent_record_offset":5536,"ancestor_record_offset":256,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3525,"record_offset":5536,"parent_record_offset":5508,"ancestor_record_offset":256,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3525,"record_offset":5508,"parent_record_offset":5472,"ancestor_record_offset":256,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3525,"record_offset":1448,"parent_record_offset":768,"ancestor_record_offset":256,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3525,"record_offset":1160,"parent_record_offset":768,"ancestor_record_offset":256,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3525,"record_offset":908,"parent_record_offset":768,"ancestor_record_offset":256,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":3525,"record_offset":2032,"parent_record_offset":2004,"ancestor_record_offset":256,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3525,"record_offset":5616,"parent_record_offset":5592,"ancestor_record_offset":256,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3525,"record_offset":8396,"parent_record_offset":8372,"ancestor_record_offset":256,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::addr_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3525,"record_offset":2228,"parent_record_offset":2204,"ancestor_record_offset":256,"inlinee":8150,"source_line":2098,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3525,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":256,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3525,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":256,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3525,"record_offset":8912,"parent_record_offset":8884,"ancestor_record_offset":256,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3525,"record_offset":4228,"parent_record_offset":4200,"ancestor_record_offset":256,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3525,"record_offset":5472,"parent_record_offset":5448,"ancestor_record_offset":256,"inlinee":5713,"source_line":57,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3525,"record_offset":4256,"parent_record_offset":4228,"ancestor_record_offset":256,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3525,"record_offset":824,"parent_record_offset":768,"ancestor_record_offset":256,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3525,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":256,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3525,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":256,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3525,"record_offset":1216,"parent_record_offset":768,"ancestor_record_offset":256,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3525,"record_offset":1296,"parent_record_offset":1272,"ancestor_record_offset":256,"inlinee":5264,"source_line":389,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3525,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":256,"inlinee":5333,"source_line":547,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3525,"record_offset":1248,"parent_record_offset":1216,"ancestor_record_offset":256,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3525,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":256,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3525,"record_offset":932,"parent_record_offset":908,"ancestor_record_offset":256,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::impl$17::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.25.0/src/f32/sse2/mat4.rs
//     evidence={"module":3525,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":256,"inlinee":10460,"source_line":1376,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::cmpeq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.25.0/src/f32/sse2/vec4.rs
//     evidence={"module":3525,"record_offset":1672,"parent_record_offset":1648,"ancestor_record_offset":256,"inlinee":10464,"source_line":250,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$2::eq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.25.0/src/f32/sse2/vec4.rs
//     evidence={"module":3525,"record_offset":1648,"parent_record_offset":1612,"ancestor_record_offset":256,"inlinee":10462,"source_line":803,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::alignment::impl$13::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/alignment.rs
//     evidence={"module":3525,"record_offset":4708,"parent_record_offset":4684,"ancestor_record_offset":256,"inlinee":10474,"source_line":37,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::alignment::impl$20::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/alignment.rs
//     evidence={"module":3525,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":256,"inlinee":10476,"source_line":50,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::angle::impl$25::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/angle.rs
//     evidence={"module":3525,"record_offset":7764,"parent_record_offset":7740,"ancestor_record_offset":256,"inlinee":10494,"source_line":11,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::background::impl$7::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/background.rs
//     evidence={"module":3525,"record_offset":7636,"parent_record_offset":7608,"ancestor_record_offset":256,"inlinee":10488,"source_line":5,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::color::impl$11::eq
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/color.rs
//     evidence={"module":3525,"record_offset":4120,"parent_record_offset":4096,"ancestor_record_offset":256,"inlinee":10413,"source_line":5,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::gradient::impl$18::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/gradient.rs
//     evidence={"module":3525,"record_offset":7740,"parent_record_offset":7716,"ancestor_record_offset":256,"inlinee":10492,"source_line":50,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::gradient::impl$7::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/gradient.rs
//     evidence={"module":3525,"record_offset":7692,"parent_record_offset":7668,"ancestor_record_offset":256,"inlinee":10490,"source_line":6,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$14::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":3525,"record_offset":7172,"parent_record_offset":7140,"ancestor_record_offset":256,"inlinee":10480,"source_line":9,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$19::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":3525,"record_offset":7204,"parent_record_offset":7172,"ancestor_record_offset":256,"inlinee":10486,"source_line":138,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$26::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":3525,"record_offset":8704,"parent_record_offset":8680,"ancestor_record_offset":256,"inlinee":10500,"source_line":170,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::pixels::impl$8::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/pixels.rs
//     evidence={"module":3525,"record_offset":4568,"parent_record_offset":4544,"ancestor_record_offset":256,"inlinee":10470,"source_line":9,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::point::impl$13::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/point.rs
//     evidence={"module":3525,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":256,"inlinee":10468,"source_line":7,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::rectangle::impl$9::eq
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3525,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":256,"inlinee":10454,"source_line":4,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::svg::impl$7::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/svg.rs
//     evidence={"module":3525,"record_offset":8220,"parent_record_offset":8196,"ancestor_record_offset":256,"inlinee":10496,"source_line":10,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::text::impl$13::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/text.rs
//     evidence={"module":3525,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":256,"inlinee":10478,"source_line":46,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::text::impl$21::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/text.rs
//     evidence={"module":3525,"record_offset":4624,"parent_record_offset":4600,"ancestor_record_offset":256,"inlinee":10472,"source_line":70,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::transformation::impl$13::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/transformation.rs
//     evidence={"module":3525,"record_offset":1588,"parent_record_offset":1564,"ancestor_record_offset":256,"inlinee":10458,"source_line":7,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_graphics::primitive::impl$4::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/graphics/src/primitive.rs
//     evidence={"module":3525,"record_offset":2796,"parent_record_offset":2744,"ancestor_record_offset":256,"inlinee":10540,"source_line":16,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::primitive::impl$4::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/primitive.rs
//     evidence={"module":3525,"record_offset":5276,"parent_record_offset":5228,"ancestor_record_offset":256,"inlinee":10482,"source_line":6,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] strict_num::impl$2::eq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"strict-num","version":"0.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/strict-num-0.1.1/src/lib.rs
//     evidence={"module":3525,"record_offset":5992,"parent_record_offset":5956,"ancestor_record_offset":256,"inlinee":10332,"source_line":137,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia::blend_mode::impl$6::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/blend_mode.rs
//     evidence={"module":3525,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":256,"inlinee":10508,"source_line":4,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia::painter::impl$10::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/painter.rs
//     evidence={"module":3525,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":256,"inlinee":10510,"source_line":22,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia::painter::impl$14::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/painter.rs
//     evidence={"module":3525,"record_offset":6904,"parent_record_offset":6880,"ancestor_record_offset":256,"inlinee":10506,"source_line":37,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::impl$12::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/lib.rs
//     evidence={"module":3525,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":256,"inlinee":10502,"source_line":70,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::path::impl$14::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/path.rs
//     evidence={"module":3525,"record_offset":5384,"parent_record_offset":5348,"ancestor_record_offset":256,"inlinee":10484,"source_line":48,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::path::impl$8::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/path.rs
//     evidence={"module":3525,"record_offset":5680,"parent_record_offset":5656,"ancestor_record_offset":256,"inlinee":10448,"source_line":18,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::rect::impl$13::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/rect.rs
//     evidence={"module":3525,"record_offset":5956,"parent_record_offset":5384,"ancestor_record_offset":256,"inlinee":10504,"source_line":214,"archive":"_pdb/B/modules/03525.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::text
// 记录形式: inline_instance, inlinee_declaration; instances=66
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xD7F1
// 首条位置: {"module":882,"inlinee":55281}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":882,"record_offset":1896,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":6980,"parent_record_offset":6804,"ancestor_record_offset":6804,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::new
//     owner=git_dependency_definition; count=24
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":882,"record_offset":6924,"parent_record_offset":6804,"ancestor_record_offset":6804,"inlinee":16847,"source_line":41,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":882,"record_offset":6952,"parent_record_offset":6804,"ancestor_record_offset":6804,"inlinee":16862,"source_line":113,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":1852,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":1852,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=51
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":7156,"parent_record_offset":7132,"ancestor_record_offset":6804,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":2592,"parent_record_offset":2408,"ancestor_record_offset":1852,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=51
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":7132,"parent_record_offset":7076,"ancestor_record_offset":6804,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":882,"record_offset":6888,"parent_record_offset":6864,"ancestor_record_offset":6804,"inlinee":6411,"source_line":87,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":2332,"parent_record_offset":2296,"ancestor_record_offset":1852,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=51
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":7104,"parent_record_offset":7076,"ancestor_record_offset":6804,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=51
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":7076,"parent_record_offset":7052,"ancestor_record_offset":6804,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":2408,"parent_record_offset":2368,"ancestor_record_offset":1852,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":2368,"parent_record_offset":2332,"ancestor_record_offset":1852,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=51
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":7052,"parent_record_offset":7028,"ancestor_record_offset":6804,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":2452,"parent_record_offset":2408,"ancestor_record_offset":1852,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":2248,"parent_record_offset":2204,"ancestor_record_offset":1852,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":2204,"parent_record_offset":2160,"ancestor_record_offset":1852,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":2116,"parent_record_offset":2072,"ancestor_record_offset":1852,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":2160,"parent_record_offset":2116,"ancestor_record_offset":1852,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":2072,"parent_record_offset":2028,"ancestor_record_offset":1852,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":882,"record_offset":2028,"parent_record_offset":1984,"ancestor_record_offset":1852,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2206,"record_offset":1784,"parent_record_offset":1736,"ancestor_record_offset":1672,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$108::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3503,"record_offset":32816,"parent_record_offset":32780,"ancestor_record_offset":32744,"inlinee":93423,"source_line":2877,"archive":"_pdb/B/modules/03503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$109::spec_to_string
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":882,"record_offset":1940,"parent_record_offset":1896,"ancestor_record_offset":1852,"inlinee":8861,"source_line":2877,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$23::fmt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2206,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":1672,"inlinee":5709,"source_line":2578,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2206,"record_offset":1736,"parent_record_offset":1704,"ancestor_record_offset":1672,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$42::spec_to_string
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":882,"record_offset":6864,"parent_record_offset":6840,"ancestor_record_offset":6804,"inlinee":6409,"source_line":2912,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":882,"record_offset":1984,"parent_record_offset":1940,"ancestor_record_offset":1852,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":882,"record_offset":2296,"parent_record_offset":2248,"ancestor_record_offset":1852,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":882,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":1852,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":882,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":1852,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":2206,"record_offset":1812,"parent_record_offset":1736,"ancestor_record_offset":1672,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$76::fmt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":2206,"record_offset":1840,"parent_record_offset":1736,"ancestor_record_offset":1672,"inlinee":6437,"source_line":2652,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":882,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":1852,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":882,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":1852,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":882,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":1852,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":882,"record_offset":2692,"parent_record_offset":2248,"ancestor_record_offset":1852,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=105
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":7004,"parent_record_offset":6980,"ancestor_record_offset":6804,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2206,"record_offset":1896,"parent_record_offset":1736,"ancestor_record_offset":1672,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/02206.jsonl.gz"}
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
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
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
// 文件级候选接口: 10; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)
// 候选参数类型: ["ref$<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::row::Row<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: iced_graphics::damage::impl$0::bounds<enum2$<iced_tiny_skia::primitive::Custom> >
// 信息 ID: 4783caaa381d0349595f9867
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 089084bcf65bc8c4f0175768: iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// [类型关系] enum2$ -> ["iced_tiny_skia::primitive::Custom"]; 字段布局 = unknown。
// [类型关系] iced_graphics::damage::impl$0::bounds -> ["enum2$<iced_tiny_skia::primitive::Custom>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::chain::Chain<core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)> > > @ 0x1a752ce -> 0x21d53d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::chain::Chain<core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)> > > @ 0x1a74e10 -> 0x21d53d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> > @ 0x21d3727 -> 0x21d53d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> > @ 0x21d3734 -> 0x21d53d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> >
// 信息 ID: ecb06b2804151bd26efb83b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 089084bcf65bc8c4f0175768: iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// [类型关系] enum2$ -> ["iced_tiny_skia::primitive::Custom"]; 字段布局 = unknown。
// [类型关系] iced_graphics::damage::regions -> ["enum2$<iced_tiny_skia::primitive::Custom>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > > @ 0x1a71a89 -> 0x21d2870 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > > > @ 0x1a737fe -> 0x21d2870 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::chain::Chain<core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)> > > @ 0x1a7515a -> 0x21d2870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::text
// 信息 ID: 4855aa251556b7ef9a12e91d
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
// [PDB 内联宿主] tg_integrity_report::controls::Controls::verification_status; module=882, inline_record=1852; 不是独立调用边。
// [PDB 内联宿主] tg_integrity_report::controls::Controls::verification_status; module=882, inline_record=5312; 不是独立调用边。
// [PDB 内联宿主] tg_integrity_report::controls::Controls::verification_status; module=882, inline_record=6804; 不是独立调用边。
// [PDB 内联宿主] tg_integrity_report::controls::Controls::verification_status; module=882, inline_record=8484; 不是独立调用边。
// [PDB 内联宿主] tg_integrity_report::controls::Controls::verification_status; module=882, inline_record=10888; 不是独立调用边。
// [PDB 内联宿主] tg_integrity_report::controls::Controls::verification_status; module=882, inline_record=12416; 不是独立调用边。
// [PDB 内联宿主] tg_integrity_report::controls::Controls::verification_status; module=882, inline_record=13476; 不是独立调用边。
// [PDB 内联宿主] tg_integrity_report::controls::Controls::verification_status; module=882, inline_record=14724; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
