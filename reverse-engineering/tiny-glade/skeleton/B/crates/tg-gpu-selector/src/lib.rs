// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tg-gpu-selector/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 188911a379da5aef80a4acd670630f9de10ea2b113b8134362216f235740d4b8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 273, 2318, 2319, 2323, 3496, 3509
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
// use tg_gpu_selector; // namespace candidate
//   package identities: [{"name":"tg-gpu-selector","source":"local_or_path","version":"0.1.0"}]
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
// 首条位置: {"module":3509,"record_offset":11636,"rva":35430560,"va":5404139680}
// 编译实例 RVAs: 0x21ca0a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1421ca0a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404139680,"raw_hex":"e8f9feffff","size":5,"soft_fail":false,"target_va":5404139680,"va":5404139938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::rectangle::Rectangle::union
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-28812102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404139680,"raw_hex":"e8ba5c48fe","size":5,"soft_fail":false,"target_va":5375328016,"va":5404140113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::transformation::_$LT$impl$u20$core..ops..arith..Mul$LT$iced_core..transformation..Transformation$GT$$u20$for$u20$iced_core..rectangle..Rectangle$GT$::mul
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-28811771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404139680,"raw_hex":"e8055e48fe","size":5,"soft_fail":false,"target_va":5375328224,"va":5404139990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3509,"record_offset":11828,"parent_record_offset":11636,"ancestor_record_offset":11636,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3509,"record_offset":12064,"parent_record_offset":11636,"ancestor_record_offset":11636,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3509,"record_offset":11944,"parent_record_offset":11636,"ancestor_record_offset":11636,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3509,"record_offset":11912,"parent_record_offset":11636,"ancestor_record_offset":11636,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3509,"record_offset":12120,"parent_record_offset":11636,"ancestor_record_offset":11636,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] iced_core::rectangle::Rectangle::expand
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3509,"record_offset":11972,"parent_record_offset":11636,"ancestor_record_offset":11636,"inlinee":10512,"source_line":152,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] iced_tiny_skia::primitive::impl$0::bounds
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/primitive.rs
//     evidence={"module":3509,"record_offset":12320,"parent_record_offset":11636,"ancestor_record_offset":11636,"inlinee":10516,"source_line":29,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3509,"record_offset":11852,"parent_record_offset":11828,"ancestor_record_offset":11636,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3509,"record_offset":12088,"parent_record_offset":12064,"ancestor_record_offset":11636,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3509,"record_offset":12228,"parent_record_offset":12148,"ancestor_record_offset":11636,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3509,"record_offset":12252,"parent_record_offset":12228,"ancestor_record_offset":11636,"inlinee":4196,"source_line":166,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3509,"record_offset":11876,"parent_record_offset":11852,"ancestor_record_offset":11636,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3509,"record_offset":12200,"parent_record_offset":12148,"ancestor_record_offset":11636,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3509,"record_offset":12148,"parent_record_offset":12120,"ancestor_record_offset":11636,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::rectangle::Rectangle::expand
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3509,"record_offset":12408,"parent_record_offset":12320,"ancestor_record_offset":11636,"inlinee":10512,"source_line":152,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] iced_graphics::damage::impl$0::bounds::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/graphics/src/damage.rs
//     evidence={"module":3509,"record_offset":12280,"parent_record_offset":12228,"ancestor_record_offset":11636,"inlinee":10514,"source_line":102,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::path::Path::bounds
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/path.rs
//     evidence={"module":3509,"record_offset":12352,"parent_record_offset":12320,"ancestor_record_offset":11636,"inlinee":10518,"source_line":69,"archive":"_pdb/B/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::rect::Rect::width
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/rect.rs
//     evidence={"module":3509,"record_offset":12380,"parent_record_offset":12320,"ancestor_record_offset":11636,"inlinee":10520,"source_line":294,"archive":"_pdb/B/modules/03509.jsonl.gz"}
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
// 首条位置: {"module":3496,"record_offset":264,"rva":35361280,"va":5404070400}
// 编译实例 RVAs: 0x21b9200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_core..image..Bytes$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28747591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e8b95849fe","size":5,"soft_fail":false,"target_va":5375325088,"va":5404072674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_graphics..text..editor..Weak$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28758839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e8c92c49fe","size":5,"soft_fail":false,"target_va":5375313472,"va":5404072306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_graphics..text..paragraph..Weak$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28749316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e8fc5149fe","size":5,"soft_fail":false,"target_va":5375322416,"va":5404071727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b6580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32517968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e8b0d00ffe","size":5,"soft_fail":false,"target_va":5371553152,"va":5404071115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b67f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32517076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e82cd40ffe","size":5,"soft_fail":false,"target_va":5371553776,"va":5404070847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140510730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30053505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e87f6b35fe","size":5,"soft_fail":false,"target_va":5374019376,"va":5404072876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28990006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e8caa545fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5404071489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a51ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7764000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e8e08789ff","size":5,"soft_fail":false,"target_va":5396306640,"va":5404070635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a520f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7763500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e8d48989ff","size":5,"soft_fail":false,"target_va":5396308208,"va":5404071703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421b9200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e891feffff","size":5,"soft_fail":false,"target_va":5404070400,"va":5404070762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421c9520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t63780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e824f90000","size":5,"soft_fail":false,"target_va":5404136736,"va":5404072951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421ca0a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t65508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e8e4ff0000","size":5,"soft_fail":false,"target_va":5404139680,"va":5404074167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7775251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e813a47600","size":5,"soft_fail":false,"target_va":5411847152,"va":5404071896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7466574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e84eee7100","size":5,"soft_fail":false,"target_va":5411540992,"va":5404074413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::border::impl$11::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-33091165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e8a31107fe","size":5,"soft_fail":false,"target_va":5370982576,"va":5404073736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::font::impl$5::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-33089694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e8621707fe","size":5,"soft_fail":false,"target_va":5370982464,"va":5404072153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::shadow::impl$4::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-33091073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e8ff1107fe","size":5,"soft_fail":false,"target_va":5370982704,"va":5404073772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::svg::impl$12::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t1956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e8a4070000","size":5,"soft_fail":false,"target_va":5404074976,"va":5404073015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e877040000","size":5,"soft_fail":false,"target_va":5404074448,"va":5404073300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_skia::shaders::impl$9::eq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e89e040000","size":5,"soft_fail":false,"target_va":5404075088,"va":5404073901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_skia_path::stroker::impl$11::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-33090724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404070400,"raw_hex":"e85c1307fe","size":5,"soft_fail":false,"target_va":5370983264,"va":5404073983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3496,"record_offset":8808,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3496,"record_offset":8964,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::ptr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3496,"record_offset":2212,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8149,"source_line":1933,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3496,"record_offset":2268,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3496,"record_offset":2012,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3496,"record_offset":468,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3496,"record_offset":1572,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3496,"record_offset":2156,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3496,"record_offset":2640,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":15461,"source_line":911,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3496,"record_offset":2100,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_direct] iced_core::rectangle::impl$9::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3496,"record_offset":9048,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10454,"source_line":4,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_direct] iced_graphics::damage::list
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/graphics/src/damage.rs
//     evidence={"module":3496,"record_offset":776,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":14003,"source_line":186,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3496,"record_offset":8868,"parent_record_offset":8844,"ancestor_record_offset":264,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3496,"record_offset":8892,"parent_record_offset":8868,"ancestor_record_offset":264,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3496,"record_offset":8844,"parent_record_offset":8808,"ancestor_record_offset":264,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$15::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3496,"record_offset":3436,"parent_record_offset":3412,"ancestor_record_offset":264,"inlinee":10456,"source_line":1822,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3496,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":264,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3496,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3496,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":264,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3496,"record_offset":2312,"parent_record_offset":2268,"ancestor_record_offset":264,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3496,"record_offset":4184,"parent_record_offset":4160,"ancestor_record_offset":264,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3496,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":264,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::ptr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3496,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":264,"inlinee":8149,"source_line":1933,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3496,"record_offset":3080,"parent_record_offset":3056,"ancestor_record_offset":264,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$47::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3496,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":264,"inlinee":10452,"source_line":3246,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$48::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3496,"record_offset":8276,"parent_record_offset":8252,"ancestor_record_offset":264,"inlinee":10498,"source_line":3265,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$49::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3496,"record_offset":3032,"parent_record_offset":3008,"ancestor_record_offset":264,"inlinee":10450,"source_line":3295,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3496,"record_offset":516,"parent_record_offset":492,"ancestor_record_offset":264,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3496,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":264,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3496,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":264,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3496,"record_offset":1192,"parent_record_offset":1168,"ancestor_record_offset":264,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":3496,"record_offset":4208,"parent_record_offset":4184,"ancestor_record_offset":264,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":3496,"record_offset":7800,"parent_record_offset":7748,"ancestor_record_offset":264,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$8::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":3496,"record_offset":7824,"parent_record_offset":7800,"ancestor_record_offset":264,"inlinee":10417,"source_line":137,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3496,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":264,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3496,"record_offset":5664,"parent_record_offset":5480,"ancestor_record_offset":264,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3496,"record_offset":7336,"parent_record_offset":7312,"ancestor_record_offset":264,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3496,"record_offset":2432,"parent_record_offset":2396,"ancestor_record_offset":264,"inlinee":8507,"source_line":1740,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3496,"record_offset":5572,"parent_record_offset":5544,"ancestor_record_offset":264,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=45
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3496,"record_offset":3008,"parent_record_offset":2804,"ancestor_record_offset":264,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3496,"record_offset":1100,"parent_record_offset":1076,"ancestor_record_offset":264,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmpeq_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3496,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":264,"inlinee":10466,"source_line":483,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3496,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":264,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3496,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":264,"inlinee":15463,"source_line":23,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3496,"record_offset":2124,"parent_record_offset":2100,"ancestor_record_offset":264,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3496,"record_offset":1052,"parent_record_offset":916,"ancestor_record_offset":264,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3496,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":264,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3496,"record_offset":5600,"parent_record_offset":5544,"ancestor_record_offset":264,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3496,"record_offset":5544,"parent_record_offset":5516,"ancestor_record_offset":264,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3496,"record_offset":5516,"parent_record_offset":5480,"ancestor_record_offset":264,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3496,"record_offset":1456,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3496,"record_offset":1168,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3496,"record_offset":916,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":3496,"record_offset":2040,"parent_record_offset":2012,"ancestor_record_offset":264,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3496,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":264,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3496,"record_offset":8404,"parent_record_offset":8380,"ancestor_record_offset":264,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::addr_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3496,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":264,"inlinee":8150,"source_line":2098,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3496,"record_offset":880,"parent_record_offset":856,"ancestor_record_offset":264,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3496,"record_offset":3128,"parent_record_offset":3104,"ancestor_record_offset":264,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3496,"record_offset":8920,"parent_record_offset":8892,"ancestor_record_offset":264,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3496,"record_offset":4236,"parent_record_offset":4208,"ancestor_record_offset":264,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3496,"record_offset":5480,"parent_record_offset":5456,"ancestor_record_offset":264,"inlinee":5713,"source_line":57,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3496,"record_offset":4264,"parent_record_offset":4236,"ancestor_record_offset":264,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3496,"record_offset":832,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3496,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":264,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3496,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":264,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3496,"record_offset":1224,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3496,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":264,"inlinee":5264,"source_line":389,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3496,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":264,"inlinee":5333,"source_line":547,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3496,"record_offset":1256,"parent_record_offset":1224,"ancestor_record_offset":264,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3496,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":264,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3496,"record_offset":940,"parent_record_offset":916,"ancestor_record_offset":264,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::impl$17::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.25.0/src/f32/sse2/mat4.rs
//     evidence={"module":3496,"record_offset":1620,"parent_record_offset":1596,"ancestor_record_offset":264,"inlinee":10460,"source_line":1376,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::cmpeq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.25.0/src/f32/sse2/vec4.rs
//     evidence={"module":3496,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":264,"inlinee":10464,"source_line":250,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$2::eq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.25.0/src/f32/sse2/vec4.rs
//     evidence={"module":3496,"record_offset":1656,"parent_record_offset":1620,"ancestor_record_offset":264,"inlinee":10462,"source_line":803,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::alignment::impl$13::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/alignment.rs
//     evidence={"module":3496,"record_offset":4716,"parent_record_offset":4692,"ancestor_record_offset":264,"inlinee":10474,"source_line":37,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::alignment::impl$20::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/alignment.rs
//     evidence={"module":3496,"record_offset":4772,"parent_record_offset":4748,"ancestor_record_offset":264,"inlinee":10476,"source_line":50,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::angle::impl$25::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/angle.rs
//     evidence={"module":3496,"record_offset":7772,"parent_record_offset":7748,"ancestor_record_offset":264,"inlinee":10494,"source_line":11,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::background::impl$7::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/background.rs
//     evidence={"module":3496,"record_offset":7644,"parent_record_offset":7616,"ancestor_record_offset":264,"inlinee":10488,"source_line":5,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::color::impl$11::eq
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/color.rs
//     evidence={"module":3496,"record_offset":4128,"parent_record_offset":4104,"ancestor_record_offset":264,"inlinee":10413,"source_line":5,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::gradient::impl$18::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/gradient.rs
//     evidence={"module":3496,"record_offset":7748,"parent_record_offset":7724,"ancestor_record_offset":264,"inlinee":10492,"source_line":50,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::gradient::impl$7::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/gradient.rs
//     evidence={"module":3496,"record_offset":7700,"parent_record_offset":7676,"ancestor_record_offset":264,"inlinee":10490,"source_line":6,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$14::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":3496,"record_offset":7180,"parent_record_offset":7148,"ancestor_record_offset":264,"inlinee":10480,"source_line":9,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$19::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":3496,"record_offset":7212,"parent_record_offset":7180,"ancestor_record_offset":264,"inlinee":10486,"source_line":138,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$26::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":3496,"record_offset":8712,"parent_record_offset":8688,"ancestor_record_offset":264,"inlinee":10500,"source_line":170,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::pixels::impl$8::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/pixels.rs
//     evidence={"module":3496,"record_offset":4576,"parent_record_offset":4552,"ancestor_record_offset":264,"inlinee":10470,"source_line":9,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::point::impl$13::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/point.rs
//     evidence={"module":3496,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":264,"inlinee":10468,"source_line":7,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::rectangle::impl$9::eq
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3496,"record_offset":2396,"parent_record_offset":2372,"ancestor_record_offset":264,"inlinee":10454,"source_line":4,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::svg::impl$7::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/svg.rs
//     evidence={"module":3496,"record_offset":8228,"parent_record_offset":8204,"ancestor_record_offset":264,"inlinee":10496,"source_line":10,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::text::impl$13::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/text.rs
//     evidence={"module":3496,"record_offset":4828,"parent_record_offset":4804,"ancestor_record_offset":264,"inlinee":10478,"source_line":46,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::text::impl$21::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/text.rs
//     evidence={"module":3496,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":264,"inlinee":10472,"source_line":70,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::transformation::impl$13::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/transformation.rs
//     evidence={"module":3496,"record_offset":1596,"parent_record_offset":1572,"ancestor_record_offset":264,"inlinee":10458,"source_line":7,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_graphics::primitive::impl$4::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/graphics/src/primitive.rs
//     evidence={"module":3496,"record_offset":2804,"parent_record_offset":2752,"ancestor_record_offset":264,"inlinee":10540,"source_line":16,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::primitive::impl$4::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/primitive.rs
//     evidence={"module":3496,"record_offset":5284,"parent_record_offset":5236,"ancestor_record_offset":264,"inlinee":10482,"source_line":6,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] strict_num::impl$2::eq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"strict-num","version":"0.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/strict-num-0.1.1/src/lib.rs
//     evidence={"module":3496,"record_offset":6000,"parent_record_offset":5964,"ancestor_record_offset":264,"inlinee":10332,"source_line":137,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia::blend_mode::impl$6::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/blend_mode.rs
//     evidence={"module":3496,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":264,"inlinee":10508,"source_line":4,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia::painter::impl$10::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/painter.rs
//     evidence={"module":3496,"record_offset":7108,"parent_record_offset":7084,"ancestor_record_offset":264,"inlinee":10510,"source_line":22,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia::painter::impl$14::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/painter.rs
//     evidence={"module":3496,"record_offset":6912,"parent_record_offset":6888,"ancestor_record_offset":264,"inlinee":10506,"source_line":37,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::impl$12::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/lib.rs
//     evidence={"module":3496,"record_offset":5920,"parent_record_offset":5896,"ancestor_record_offset":264,"inlinee":10502,"source_line":70,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::path::impl$14::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/path.rs
//     evidence={"module":3496,"record_offset":5392,"parent_record_offset":5356,"ancestor_record_offset":264,"inlinee":10484,"source_line":48,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::path::impl$8::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/path.rs
//     evidence={"module":3496,"record_offset":5688,"parent_record_offset":5664,"ancestor_record_offset":264,"inlinee":10448,"source_line":18,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::rect::impl$13::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/rect.rs
//     evidence={"module":3496,"record_offset":5964,"parent_record_offset":5392,"ancestor_record_offset":264,"inlinee":10504,"source_line":214,"archive":"_pdb/B/modules/03496.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::run_gpu_selection_gui
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":273,"record_offset":1656,"rva":6491680,"va":5375200800}
// 编译实例 RVAs: 0x630e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140def030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8118744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375200800,"raw_hex":"e8d8e17b00","size":5,"soft_fail":false,"target_va":5383319600,"va":5375200851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140defad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8121315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375200800,"raw_hex":"e8e3eb7b00","size":5,"soft_fail":false,"target_va":5383322320,"va":5375201000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375200800,"raw_hex":"e8fb080000","size":5,"soft_fail":false,"target_va":5375203392,"va":5375201088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$rhapsody..raw..init..UnavailableGraphicsDevice$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8114939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375200800,"raw_hex":"e8fbd27b00","size":5,"soft_fail":false,"target_va":5383316048,"va":5375201104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<tg_gpu_selector::gpu_selection::GpuSelectionReason> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375200800,"raw_hex":"e803fdffff","size":5,"soft_fail":false,"target_va":5375200336,"va":5375201096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":273,"record_offset":1852,"parent_record_offset":1656,"ancestor_record_offset":1656,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00273.jsonl.gz"}
//   [pdb_inline_direct] tg_gpu_selector::gpu_selection::Controls::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/gpu_selection.rs
//     evidence={"module":273,"record_offset":1784,"parent_record_offset":1656,"ancestor_record_offset":1656,"inlinee":17046,"source_line":72,"archive":"_pdb/B/modules/00273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":273,"record_offset":1876,"parent_record_offset":1852,"ancestor_record_offset":1656,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":273,"record_offset":1820,"parent_record_offset":1784,"ancestor_record_offset":1656,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":273,"record_offset":1912,"parent_record_offset":1876,"ancestor_record_offset":1656,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":273,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1656,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":273,"record_offset":1968,"parent_record_offset":1876,"ancestor_record_offset":1656,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00273.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::run_vulkan_configuration_gui
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":273,"record_offset":2008,"rva":6492000,"va":5375201120}
// 编译实例 RVAs: 0x630f60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df16d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8128288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375201120,"raw_hex":"e820077c00","size":5,"soft_fail":false,"target_va":5383329488,"va":5375201195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375201120,"raw_hex":"e8f2070000","size":5,"soft_fail":false,"target_va":5375203392,"va":5375201353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_gpu_selector::vulkan_config::Controls::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8136268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375201120,"raw_hex":"e84c267c00","size":5,"soft_fail":false,"target_va":5383337440,"va":5375201167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":273,"record_offset":2208,"parent_record_offset":2008,"ancestor_record_offset":2008,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00273.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":273,"record_offset":2148,"parent_record_offset":2008,"ancestor_record_offset":2008,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00273.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":273,"record_offset":2180,"parent_record_offset":2008,"ancestor_record_offset":2008,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":273,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":2008,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":273,"record_offset":2268,"parent_record_offset":2232,"ancestor_record_offset":2008,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":273,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":2008,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":273,"record_offset":2324,"parent_record_offset":2232,"ancestor_record_offset":2008,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00273.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] tg_gpu_selector::Workarounds
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds>", "constructor": "tuple$", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayerAllowlist", "tg_gpu_selector::Workarounds"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> >", "constructor": "core::option::Option", "arguments": ["tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> > >", "constructor": "enum2$", "arguments": ["core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error>", "constructor": "core::result::Result", "arguments": ["enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> > >", "anyhow::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 文件级候选接口: 9; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)
// 候选参数类型: ["ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"]
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
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::chain::Chain<core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)> > > @ 0x1a526be -> 0x21ca0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::chain::Chain<core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)> > > @ 0x1a52200 -> 0x21ca0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> > @ 0x21ba0b7 -> 0x21ca0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> > @ 0x21ba0c4 -> 0x21ca0a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> >
// 信息 ID: ecb06b2804151bd26efb83b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 089084bcf65bc8c4f0175768: iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// [类型关系] enum2$ -> ["iced_tiny_skia::primitive::Custom"]; 字段布局 = unknown。
// [类型关系] iced_graphics::damage::regions -> ["enum2$<iced_tiny_skia::primitive::Custom>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > > @ 0x1a4f929 -> 0x21b9200 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > > > @ 0x1a51dde -> 0x21b9200 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::chain::Chain<core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)> > > @ 0x1a5254a -> 0x21b9200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::run_gpu_selection_gui
// 信息 ID: 8dd7e2a7f2cf717b34afde36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x598318 -> 0x630e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x595f43 -> 0x630e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::run_vulkan_configuration_gui
// 信息 ID: 5478b3ed0e96c387eaca1985
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::vulkan_config_main::run_vulkan_configuration_app @ 0x3e7b1d -> 0x630f60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
