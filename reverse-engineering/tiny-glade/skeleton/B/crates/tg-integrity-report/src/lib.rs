// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tg-integrity-report/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e44eec0b391eb7103a3dd22c1cd3571e409ee694066ba0f86b698515a51a647e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 268, 2199, 3401, 3404
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
// use tg_integrity_report; // namespace candidate
//   package identities: [{"name":"tg-integrity-report","source":"local_or_path","version":"0.1.0"}]
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
// 首条位置: {"module":3404,"record_offset":11640,"rva":35179104,"va":5403888224}
// 编译实例 RVAs: 0x218ca60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14218ca60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403888224,"raw_hex":"e8f9feffff","size":5,"soft_fail":false,"target_va":5403888224,"va":5403888482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::rectangle::Rectangle::union
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-28560646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403888224,"raw_hex":"e8fa324cfe","size":5,"soft_fail":false,"target_va":5375328016,"va":5403888657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::transformation::_$LT$impl$u20$core..ops..arith..Mul$LT$iced_core..transformation..Transformation$GT$$u20$for$u20$iced_core..rectangle..Rectangle$GT$::mul
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-28560315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403888224,"raw_hex":"e845344cfe","size":5,"soft_fail":false,"target_va":5375328224,"va":5403888534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3404,"record_offset":11832,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3404,"record_offset":12068,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3404,"record_offset":11948,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3404,"record_offset":11916,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3404,"record_offset":12124,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_direct] iced_core::rectangle::Rectangle::expand
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3404,"record_offset":11976,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":10512,"source_line":152,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_direct] iced_tiny_skia::primitive::impl$0::bounds
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/primitive.rs
//     evidence={"module":3404,"record_offset":12324,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":10516,"source_line":29,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3404,"record_offset":11856,"parent_record_offset":11832,"ancestor_record_offset":11640,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3404,"record_offset":12092,"parent_record_offset":12068,"ancestor_record_offset":11640,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3404,"record_offset":12232,"parent_record_offset":12152,"ancestor_record_offset":11640,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3404,"record_offset":12256,"parent_record_offset":12232,"ancestor_record_offset":11640,"inlinee":4196,"source_line":166,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3404,"record_offset":11880,"parent_record_offset":11856,"ancestor_record_offset":11640,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3404,"record_offset":12204,"parent_record_offset":12152,"ancestor_record_offset":11640,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3404,"record_offset":12152,"parent_record_offset":12124,"ancestor_record_offset":11640,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::rectangle::Rectangle::expand
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3404,"record_offset":12412,"parent_record_offset":12324,"ancestor_record_offset":11640,"inlinee":10512,"source_line":152,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_transitive] iced_graphics::damage::impl$0::bounds::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/graphics/src/damage.rs
//     evidence={"module":3404,"record_offset":12284,"parent_record_offset":12232,"ancestor_record_offset":11640,"inlinee":10514,"source_line":102,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::path::Path::bounds
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/path.rs
//     evidence={"module":3404,"record_offset":12356,"parent_record_offset":12324,"ancestor_record_offset":11640,"inlinee":10518,"source_line":69,"archive":"_pdb/B/modules/03404.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::rect::Rect::width
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/rect.rs
//     evidence={"module":3404,"record_offset":12384,"parent_record_offset":12324,"ancestor_record_offset":11640,"inlinee":10520,"source_line":294,"archive":"_pdb/B/modules/03404.jsonl.gz"}
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
// 首条位置: {"module":3401,"record_offset":268,"rva":35169872,"va":5403878992}
// 编译实例 RVAs: 0x218a650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_core..image..Bytes$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28556183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e869444cfe","size":5,"soft_fail":false,"target_va":5375325088,"va":5403881266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_graphics..text..editor..Weak$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28567431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e879184cfe","size":5,"soft_fail":false,"target_va":5375313472,"va":5403880898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_graphics..text..paragraph..Weak$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28557908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e8ac3d4cfe","size":5,"soft_fail":false,"target_va":5375322416,"va":5403880319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b6580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32326560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e860bc12fe","size":5,"soft_fail":false,"target_va":5371553152,"va":5403879707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b67f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32325668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e8dcbf12fe","size":5,"soft_fail":false,"target_va":5371553776,"va":5403879439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140510730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29862097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e82f5738fe","size":5,"soft_fail":false,"target_va":5374019376,"va":5403881468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28798598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e87a9148fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5403880081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419d4970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8084944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e830a284ff","size":5,"soft_fail":false,"target_va":5395794288,"va":5403879227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419d4f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8084444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e824a484ff","size":5,"soft_fail":false,"target_va":5395795856,"va":5403880295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14218a650
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e891feffff","size":5,"soft_fail":false,"target_va":5403878992,"va":5403879354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14218bee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e8940e0000","size":5,"soft_fail":false,"target_va":5403885280,"va":5403881543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14218ca60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e854150000","size":5,"soft_fail":false,"target_va":5403888224,"va":5403882759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7966659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e8c38f7900","size":5,"soft_fail":false,"target_va":5411847152,"va":5403880488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7657982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e8fed97400","size":5,"soft_fail":false,"target_va":5411540992,"va":5403883005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::border::impl$11::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-32899757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e853fd09fe","size":5,"soft_fail":false,"target_va":5370982576,"va":5403882328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::font::impl$5::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-32898286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e812030afe","size":5,"soft_fail":false,"target_va":5370982464,"va":5403880745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::shadow::impl$4::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-32899665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e8affd09fe","size":5,"soft_fail":false,"target_va":5370982704,"va":5403882364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::svg::impl$12::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t1956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e8a4070000","size":5,"soft_fail":false,"target_va":5403883568,"va":5403881607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e877040000","size":5,"soft_fail":false,"target_va":5403883040,"va":5403881892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_skia::shaders::impl$9::eq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e89e040000","size":5,"soft_fail":false,"target_va":5403883680,"va":5403882493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_skia_path::stroker::impl$11::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32899316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403878992,"raw_hex":"e80cff09fe","size":5,"soft_fail":false,"target_va":5370983264,"va":5403882575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3401,"record_offset":8812,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3401,"record_offset":8968,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::ptr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3401,"record_offset":2216,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8149,"source_line":1933,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3401,"record_offset":2272,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3401,"record_offset":2016,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3401,"record_offset":472,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3401,"record_offset":1576,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3401,"record_offset":2160,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3401,"record_offset":2644,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":15461,"source_line":911,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3401,"record_offset":2104,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_direct] iced_core::rectangle::impl$9::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3401,"record_offset":9052,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":10454,"source_line":4,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_direct] iced_graphics::damage::list
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/graphics/src/damage.rs
//     evidence={"module":3401,"record_offset":780,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":14003,"source_line":186,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3401,"record_offset":8872,"parent_record_offset":8848,"ancestor_record_offset":268,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3401,"record_offset":8896,"parent_record_offset":8872,"ancestor_record_offset":268,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3401,"record_offset":8848,"parent_record_offset":8812,"ancestor_record_offset":268,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$15::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3401,"record_offset":3440,"parent_record_offset":3416,"ancestor_record_offset":268,"inlinee":10456,"source_line":1822,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3401,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":268,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3401,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":268,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3401,"record_offset":568,"parent_record_offset":544,"ancestor_record_offset":268,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3401,"record_offset":2316,"parent_record_offset":2272,"ancestor_record_offset":268,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3401,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":268,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3401,"record_offset":3108,"parent_record_offset":3084,"ancestor_record_offset":268,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::ptr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3401,"record_offset":8304,"parent_record_offset":8280,"ancestor_record_offset":268,"inlinee":8149,"source_line":1933,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3401,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":268,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$47::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3401,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":268,"inlinee":10452,"source_line":3246,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$48::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3401,"record_offset":8280,"parent_record_offset":8256,"ancestor_record_offset":268,"inlinee":10498,"source_line":3265,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$49::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3401,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":268,"inlinee":10450,"source_line":3295,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3401,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":268,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3401,"record_offset":496,"parent_record_offset":472,"ancestor_record_offset":268,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3401,"record_offset":2340,"parent_record_offset":2316,"ancestor_record_offset":268,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3401,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":268,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":3401,"record_offset":4212,"parent_record_offset":4188,"ancestor_record_offset":268,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":3401,"record_offset":7804,"parent_record_offset":7752,"ancestor_record_offset":268,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$8::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":3401,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":268,"inlinee":10417,"source_line":137,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3401,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":268,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3401,"record_offset":5668,"parent_record_offset":5484,"ancestor_record_offset":268,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3401,"record_offset":7340,"parent_record_offset":7316,"ancestor_record_offset":268,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3401,"record_offset":2436,"parent_record_offset":2400,"ancestor_record_offset":268,"inlinee":8507,"source_line":1740,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3401,"record_offset":5576,"parent_record_offset":5548,"ancestor_record_offset":268,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=45
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3401,"record_offset":3012,"parent_record_offset":2808,"ancestor_record_offset":268,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3401,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":268,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmpeq_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3401,"record_offset":1708,"parent_record_offset":1684,"ancestor_record_offset":268,"inlinee":10466,"source_line":483,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3401,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":268,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3401,"record_offset":2668,"parent_record_offset":2644,"ancestor_record_offset":268,"inlinee":15463,"source_line":23,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3401,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":268,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3401,"record_offset":1056,"parent_record_offset":920,"ancestor_record_offset":268,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3401,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":268,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3401,"record_offset":5604,"parent_record_offset":5548,"ancestor_record_offset":268,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3401,"record_offset":5548,"parent_record_offset":5520,"ancestor_record_offset":268,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3401,"record_offset":5520,"parent_record_offset":5484,"ancestor_record_offset":268,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3401,"record_offset":1460,"parent_record_offset":780,"ancestor_record_offset":268,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3401,"record_offset":1172,"parent_record_offset":780,"ancestor_record_offset":268,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3401,"record_offset":920,"parent_record_offset":780,"ancestor_record_offset":268,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":3401,"record_offset":2044,"parent_record_offset":2016,"ancestor_record_offset":268,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3401,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":268,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3401,"record_offset":8408,"parent_record_offset":8384,"ancestor_record_offset":268,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::addr_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3401,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":268,"inlinee":8150,"source_line":2098,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3401,"record_offset":884,"parent_record_offset":860,"ancestor_record_offset":268,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3401,"record_offset":3132,"parent_record_offset":3108,"ancestor_record_offset":268,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3401,"record_offset":8924,"parent_record_offset":8896,"ancestor_record_offset":268,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3401,"record_offset":4240,"parent_record_offset":4212,"ancestor_record_offset":268,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3401,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":268,"inlinee":5713,"source_line":57,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3401,"record_offset":4268,"parent_record_offset":4240,"ancestor_record_offset":268,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3401,"record_offset":836,"parent_record_offset":780,"ancestor_record_offset":268,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3401,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":268,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3401,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":268,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3401,"record_offset":1228,"parent_record_offset":780,"ancestor_record_offset":268,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3401,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":268,"inlinee":5264,"source_line":389,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3401,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":268,"inlinee":5333,"source_line":547,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3401,"record_offset":1260,"parent_record_offset":1228,"ancestor_record_offset":268,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3401,"record_offset":860,"parent_record_offset":836,"ancestor_record_offset":268,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3401,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":268,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::impl$17::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.25.0/src/f32/sse2/mat4.rs
//     evidence={"module":3401,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":268,"inlinee":10460,"source_line":1376,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::cmpeq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.25.0/src/f32/sse2/vec4.rs
//     evidence={"module":3401,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":268,"inlinee":10464,"source_line":250,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$2::eq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.25.0/src/f32/sse2/vec4.rs
//     evidence={"module":3401,"record_offset":1660,"parent_record_offset":1624,"ancestor_record_offset":268,"inlinee":10462,"source_line":803,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::alignment::impl$13::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/alignment.rs
//     evidence={"module":3401,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":268,"inlinee":10474,"source_line":37,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::alignment::impl$20::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/alignment.rs
//     evidence={"module":3401,"record_offset":4776,"parent_record_offset":4752,"ancestor_record_offset":268,"inlinee":10476,"source_line":50,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::angle::impl$25::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/angle.rs
//     evidence={"module":3401,"record_offset":7776,"parent_record_offset":7752,"ancestor_record_offset":268,"inlinee":10494,"source_line":11,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::background::impl$7::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/background.rs
//     evidence={"module":3401,"record_offset":7648,"parent_record_offset":7620,"ancestor_record_offset":268,"inlinee":10488,"source_line":5,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::color::impl$11::eq
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/color.rs
//     evidence={"module":3401,"record_offset":4132,"parent_record_offset":4108,"ancestor_record_offset":268,"inlinee":10413,"source_line":5,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::gradient::impl$18::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/gradient.rs
//     evidence={"module":3401,"record_offset":7752,"parent_record_offset":7728,"ancestor_record_offset":268,"inlinee":10492,"source_line":50,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::gradient::impl$7::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/gradient.rs
//     evidence={"module":3401,"record_offset":7704,"parent_record_offset":7680,"ancestor_record_offset":268,"inlinee":10490,"source_line":6,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$14::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":3401,"record_offset":7184,"parent_record_offset":7152,"ancestor_record_offset":268,"inlinee":10480,"source_line":9,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$19::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":3401,"record_offset":7216,"parent_record_offset":7184,"ancestor_record_offset":268,"inlinee":10486,"source_line":138,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$26::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":3401,"record_offset":8716,"parent_record_offset":8692,"ancestor_record_offset":268,"inlinee":10500,"source_line":170,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::pixels::impl$8::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/pixels.rs
//     evidence={"module":3401,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":268,"inlinee":10470,"source_line":9,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::point::impl$13::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/point.rs
//     evidence={"module":3401,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":268,"inlinee":10468,"source_line":7,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::rectangle::impl$9::eq
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3401,"record_offset":2400,"parent_record_offset":2376,"ancestor_record_offset":268,"inlinee":10454,"source_line":4,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::svg::impl$7::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/svg.rs
//     evidence={"module":3401,"record_offset":8232,"parent_record_offset":8208,"ancestor_record_offset":268,"inlinee":10496,"source_line":10,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::text::impl$13::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/text.rs
//     evidence={"module":3401,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":268,"inlinee":10478,"source_line":46,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::text::impl$21::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/text.rs
//     evidence={"module":3401,"record_offset":4636,"parent_record_offset":4612,"ancestor_record_offset":268,"inlinee":10472,"source_line":70,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::transformation::impl$13::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/transformation.rs
//     evidence={"module":3401,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":268,"inlinee":10458,"source_line":7,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_graphics::primitive::impl$4::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/graphics/src/primitive.rs
//     evidence={"module":3401,"record_offset":2808,"parent_record_offset":2756,"ancestor_record_offset":268,"inlinee":10540,"source_line":16,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::primitive::impl$4::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/primitive.rs
//     evidence={"module":3401,"record_offset":5288,"parent_record_offset":5240,"ancestor_record_offset":268,"inlinee":10482,"source_line":6,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] strict_num::impl$2::eq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"strict-num","version":"0.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/strict-num-0.1.1/src/lib.rs
//     evidence={"module":3401,"record_offset":6004,"parent_record_offset":5968,"ancestor_record_offset":268,"inlinee":10332,"source_line":137,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia::blend_mode::impl$6::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/blend_mode.rs
//     evidence={"module":3401,"record_offset":6940,"parent_record_offset":6916,"ancestor_record_offset":268,"inlinee":10508,"source_line":4,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia::painter::impl$10::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/painter.rs
//     evidence={"module":3401,"record_offset":7112,"parent_record_offset":7088,"ancestor_record_offset":268,"inlinee":10510,"source_line":22,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia::painter::impl$14::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/painter.rs
//     evidence={"module":3401,"record_offset":6916,"parent_record_offset":6892,"ancestor_record_offset":268,"inlinee":10506,"source_line":37,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::impl$12::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/lib.rs
//     evidence={"module":3401,"record_offset":5924,"parent_record_offset":5900,"ancestor_record_offset":268,"inlinee":10502,"source_line":70,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::path::impl$14::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/path.rs
//     evidence={"module":3401,"record_offset":5396,"parent_record_offset":5360,"ancestor_record_offset":268,"inlinee":10484,"source_line":48,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::path::impl$8::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/path.rs
//     evidence={"module":3401,"record_offset":5692,"parent_record_offset":5668,"ancestor_record_offset":268,"inlinee":10448,"source_line":18,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::rect::impl$13::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/rect.rs
//     evidence={"module":3401,"record_offset":5968,"parent_record_offset":5396,"ancestor_record_offset":268,"inlinee":10504,"source_line":214,"archive":"_pdb/B/modules/03401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::run_integrity_report_gui
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":268,"record_offset":268,"rva":6482128,"va":5375191248}
// 编译实例 RVAs: 0x62e8d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140dcbee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7984580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375191248,"raw_hex":"e8c4d57900","size":5,"soft_fail":false,"target_va":5383175904,"va":5375191319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tg_integrity_report::controls::Controls::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-integrity-report","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-integrity-report/src/controls.rs
//     evidence={"module":268,"record_offset":400,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":16949,"source_line":41,"archive":"_pdb/B/modules/00268.jsonl.gz"}
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
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)
// 候选参数类型: ["ref$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"]

// ------------------------------------------------------------------
// 函数: iced_graphics::damage::impl$0::bounds<enum2$<iced_tiny_skia::primitive::Custom> >
// 信息 ID: 4783caaa381d0349595f9867
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 089084bcf65bc8c4f0175768: iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// [类型关系] enum2$ -> ["iced_tiny_skia::primitive::Custom"]; 字段布局 = unknown。
// [类型关系] iced_graphics::damage::impl$0::bounds -> ["enum2$<iced_tiny_skia::primitive::Custom>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::chain::Chain<core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)> > > @ 0x19d555e -> 0x218ca60 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::chain::Chain<core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)> > > @ 0x19d50a0 -> 0x218ca60 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> > @ 0x218b507 -> 0x218ca60 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> > @ 0x218b514 -> 0x218ca60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> >
// 信息 ID: ecb06b2804151bd26efb83b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 089084bcf65bc8c4f0175768: iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// [类型关系] enum2$ -> ["iced_tiny_skia::primitive::Custom"]; 字段布局 = unknown。
// [类型关系] iced_graphics::damage::regions -> ["enum2$<iced_tiny_skia::primitive::Custom>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > > @ 0x19d2dd9 -> 0x218a650 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > > > @ 0x19d4c7e -> 0x218a650 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::chain::Chain<core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)> > > @ 0x19d53ea -> 0x218a650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_integrity_report::run_integrity_report_gui
// 信息 ID: 6534cf02f900227d1bb48a81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::integrity_report_main::verify_package_integrity_and_show_report @ 0x256364 -> 0x62e8d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
