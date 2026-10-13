// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tg-gpu-selector/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 346ae844e5a446ee109bc524fc978070575d5d52af76a9135bfaa4da1dd169b7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 273, 975, 984, 990, 2467, 2483, 3807, 3808
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
// 首条位置: {"module":3808,"record_offset":11948,"rva":42171600,"va":5410880720}
// 编译实例 RVAs: 0x2837cd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142837cd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410880720,"raw_hex":"e8f7feffff","size":5,"soft_fail":false,"target_va":5410880720,"va":5410880980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::transformation::_$LT$impl$u20$core..ops..arith..Mul$LT$iced_core..transformation..Transformation$GT$$u20$for$u20$iced_core..rectangle..Rectangle$GT$::mul
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-34930173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410880720,"raw_hex":"e80302ebfd","size":5,"soft_fail":false,"target_va":5375950864,"va":5410881032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3808,"record_offset":12140,"parent_record_offset":11948,"ancestor_record_offset":11948,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3808,"record_offset":12376,"parent_record_offset":11948,"ancestor_record_offset":11948,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3808,"record_offset":12252,"parent_record_offset":11948,"ancestor_record_offset":11948,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3808,"record_offset":12224,"parent_record_offset":11948,"ancestor_record_offset":11948,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3808,"record_offset":12432,"parent_record_offset":11948,"ancestor_record_offset":11948,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] iced_core::rectangle::Rectangle::expand
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3808,"record_offset":12280,"parent_record_offset":11948,"ancestor_record_offset":11948,"inlinee":5474,"source_line":152,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] iced_tiny_skia::primitive::impl$0::bounds
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/primitive.rs
//     evidence={"module":3808,"record_offset":12740,"parent_record_offset":11948,"ancestor_record_offset":11948,"inlinee":16594,"source_line":29,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3808,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":11948,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3808,"record_offset":12400,"parent_record_offset":12376,"ancestor_record_offset":11948,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3808,"record_offset":12692,"parent_record_offset":12620,"ancestor_record_offset":11948,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3808,"record_offset":12660,"parent_record_offset":12620,"ancestor_record_offset":11948,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3808,"record_offset":12540,"parent_record_offset":12460,"ancestor_record_offset":11948,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3808,"record_offset":12564,"parent_record_offset":12540,"ancestor_record_offset":11948,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3808,"record_offset":12188,"parent_record_offset":12164,"ancestor_record_offset":11948,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3808,"record_offset":12512,"parent_record_offset":12460,"ancestor_record_offset":11948,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3808,"record_offset":12460,"parent_record_offset":12432,"ancestor_record_offset":11948,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::rectangle::Rectangle::expand
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3808,"record_offset":12828,"parent_record_offset":12740,"ancestor_record_offset":11948,"inlinee":5474,"source_line":152,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::rectangle::Rectangle::union
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3808,"record_offset":12620,"parent_record_offset":12592,"ancestor_record_offset":11948,"inlinee":16592,"source_line":123,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] iced_graphics::damage::impl$0::bounds::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/graphics/src/damage.rs
//     evidence={"module":3808,"record_offset":12592,"parent_record_offset":12540,"ancestor_record_offset":11948,"inlinee":16591,"source_line":102,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::path::Path::bounds
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/path.rs
//     evidence={"module":3808,"record_offset":12772,"parent_record_offset":12740,"ancestor_record_offset":11948,"inlinee":16596,"source_line":69,"archive":"_pdb/C/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::rect::Rect::width
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/rect.rs
//     evidence={"module":3808,"record_offset":12800,"parent_record_offset":12740,"ancestor_record_offset":11948,"inlinee":16598,"source_line":294,"archive":"_pdb/C/modules/03808.jsonl.gz"}
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
// 首条位置: {"module":3807,"record_offset":272,"rva":42161984,"va":5410871104}
// 编译实例 RVAs: 0x2835740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::components
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-26221388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e8b4e46ffe","size":5,"soft_fail":false,"target_va":5384653152,"va":5410874535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_core..image..Bytes$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-34923296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e8e01cebfd","size":5,"soft_fail":false,"target_va":5375950304,"va":5410873595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_graphics..text..editor..Weak$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-34932195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e81dfaeafd","size":5,"soft_fail":false,"target_va":5375940768,"va":5410872958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_graphics..text..paragraph..Weak$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-34929504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e8a004ebfd","size":5,"soft_fail":false,"target_va":5375942880,"va":5410872379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-35193476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e87cfde6fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5410872159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42149957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e8bbd77cfd","size":5,"soft_fail":false,"target_va":5368722192,"va":5410872144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fa670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-39039891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e86d4cacfd","size":5,"soft_fail":false,"target_va":5371831920,"va":5410871806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fa790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-39039342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e8924eacfd","size":5,"soft_fail":false,"target_va":5371832208,"va":5410871545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405a9550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36228224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e88033d7fd","size":5,"soft_fail":false,"target_va":5374645584,"va":5410873803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141f59790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9290904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e8683b72ff","size":5,"soft_fail":false,"target_va":5401581456,"va":5410872355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141f5a190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9287329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e85f4972ff","size":5,"soft_fail":false,"target_va":5401584016,"va":5410871340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142835740
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e890feffff","size":5,"soft_fail":false,"target_va":5410871104,"va":5410871467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142837060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e8530e0000","size":5,"soft_fail":false,"target_va":5410877536,"va":5410873864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142837cd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e8e7150000","size":5,"soft_fail":false,"target_va":5410880720,"va":5410875108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6184487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e8275e5e00","size":5,"soft_fail":false,"target_va":5417057040,"va":5410872548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5745411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e803ab5700","size":5,"soft_fail":false,"target_va":5416620769,"va":5410875353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::iter::traits::iterator::Iterator::eq_by
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2839517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e8dd532b00","size":5,"soft_fail":false,"target_va":5413714320,"va":5410874798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::border::impl$12::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-36500881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e86f0ad3fd","size":5,"soft_fail":false,"target_va":5374374160,"va":5410875036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::font::impl$6::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-36498762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e8b612d3fd","size":5,"soft_fail":false,"target_va":5374374048,"va":5410872805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::shadow::impl$5::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-36500785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e8cf0ad3fd","size":5,"soft_fail":false,"target_va":5374374288,"va":5410875068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::svg::impl$12::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t1426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e892050000","size":5,"soft_fail":false,"target_va":5410875376,"va":5410873945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_skia::shaders::impl$10::eq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e81b090000","size":5,"soft_fail":false,"target_va":5410875776,"va":5410873440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_skia_path::stroker::impl$11::eq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-36498215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410871104,"raw_hex":"e8d914d3fd","size":5,"soft_fail":false,"target_va":5374375280,"va":5410873490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3807,"record_offset":9428,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3807,"record_offset":9244,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::ptr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3807,"record_offset":2168,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7951,"source_line":2143,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3807,"record_offset":2224,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3807,"record_offset":1940,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3807,"record_offset":476,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3807,"record_offset":1500,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3807,"record_offset":2112,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3807,"record_offset":2648,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":16011,"source_line":957,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3807,"record_offset":2056,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_direct] iced_core::rectangle::impl$9::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3807,"record_offset":9512,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":16010,"source_line":4,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_direct] iced_graphics::damage::list
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/graphics/src/damage.rs
//     evidence={"module":3807,"record_offset":784,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":16004,"source_line":186,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3807,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":272,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3807,"record_offset":9352,"parent_record_offset":9328,"ancestor_record_offset":272,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3807,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":272,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3807,"record_offset":9304,"parent_record_offset":9280,"ancestor_record_offset":272,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3807,"record_offset":2320,"parent_record_offset":2224,"ancestor_record_offset":272,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$18::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3807,"record_offset":3440,"parent_record_offset":3416,"ancestor_record_offset":272,"inlinee":16021,"source_line":2086,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3807,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3807,"record_offset":596,"parent_record_offset":572,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3807,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3807,"record_offset":2264,"parent_record_offset":2224,"ancestor_record_offset":272,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3807,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":272,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3807,"record_offset":3108,"parent_record_offset":3084,"ancestor_record_offset":272,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::ptr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3807,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":272,"inlinee":7951,"source_line":2143,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3807,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":272,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$53::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3807,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":272,"inlinee":16019,"source_line":3506,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$54::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3807,"record_offset":7956,"parent_record_offset":7932,"ancestor_record_offset":272,"inlinee":16065,"source_line":3525,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$55::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3807,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":272,"inlinee":16017,"source_line":3555,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3807,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":272,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3807,"record_offset":500,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3807,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":272,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3807,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":272,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3807,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":272,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3807,"record_offset":4284,"parent_record_offset":4236,"ancestor_record_offset":272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":3807,"record_offset":4212,"parent_record_offset":4188,"ancestor_record_offset":272,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":3807,"record_offset":8424,"parent_record_offset":8372,"ancestor_record_offset":272,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$8::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":3807,"record_offset":8448,"parent_record_offset":8424,"ancestor_record_offset":272,"inlinee":16063,"source_line":143,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3807,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":272,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3807,"record_offset":5884,"parent_record_offset":5848,"ancestor_record_offset":272,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3807,"record_offset":7776,"parent_record_offset":7752,"ancestor_record_offset":272,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3807,"record_offset":2412,"parent_record_offset":2376,"ancestor_record_offset":272,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3807,"record_offset":3012,"parent_record_offset":2808,"ancestor_record_offset":272,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3807,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":272,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmpeq_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3807,"record_offset":1632,"parent_record_offset":1608,"ancestor_record_offset":272,"inlinee":11124,"source_line":503,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3807,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":272,"inlinee":8137,"source_line":36,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3807,"record_offset":2672,"parent_record_offset":2648,"ancestor_record_offset":272,"inlinee":16013,"source_line":23,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3807,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":272,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":3807,"record_offset":7680,"parent_record_offset":7648,"ancestor_record_offset":272,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":3807,"record_offset":1064,"parent_record_offset":928,"ancestor_record_offset":272,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":3807,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":272,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3807,"record_offset":1384,"parent_record_offset":784,"ancestor_record_offset":272,"inlinee":8135,"source_line":507,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3807,"record_offset":1180,"parent_record_offset":784,"ancestor_record_offset":272,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3807,"record_offset":7648,"parent_record_offset":7412,"ancestor_record_offset":272,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3807,"record_offset":928,"parent_record_offset":784,"ancestor_record_offset":272,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":3807,"record_offset":1968,"parent_record_offset":1940,"ancestor_record_offset":272,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":3807,"record_offset":1992,"parent_record_offset":1968,"ancestor_record_offset":272,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3807,"record_offset":7504,"parent_record_offset":7480,"ancestor_record_offset":272,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3807,"record_offset":8832,"parent_record_offset":8808,"ancestor_record_offset":272,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::addr_eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3807,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":272,"inlinee":7952,"source_line":2443,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3807,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3807,"record_offset":3132,"parent_record_offset":3108,"ancestor_record_offset":272,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3807,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":272,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal_same_length
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3807,"record_offset":5848,"parent_record_offset":5756,"ancestor_record_offset":272,"inlinee":6223,"source_line":122,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3807,"record_offset":4628,"parent_record_offset":4236,"ancestor_record_offset":272,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3807,"record_offset":844,"parent_record_offset":784,"ancestor_record_offset":272,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3807,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":272,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3807,"record_offset":1236,"parent_record_offset":784,"ancestor_record_offset":272,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3807,"record_offset":1268,"parent_record_offset":1236,"ancestor_record_offset":272,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3807,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":272,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3807,"record_offset":952,"parent_record_offset":928,"ancestor_record_offset":272,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::impl$17::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.25.0/src/f32/sse2/mat4.rs
//     evidence={"module":3807,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":272,"inlinee":16008,"source_line":1376,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::cmpeq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.25.0/src/f32/sse2/vec4.rs
//     evidence={"module":3807,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":272,"inlinee":11123,"source_line":250,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$2::eq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.25.0/src/f32/sse2/vec4.rs
//     evidence={"module":3807,"record_offset":1584,"parent_record_offset":1548,"ancestor_record_offset":272,"inlinee":11121,"source_line":803,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::alignment::impl$15::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/alignment.rs
//     evidence={"module":3807,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":272,"inlinee":16031,"source_line":37,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::alignment::impl$23::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/alignment.rs
//     evidence={"module":3807,"record_offset":5128,"parent_record_offset":5104,"ancestor_record_offset":272,"inlinee":16033,"source_line":50,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::angle::impl$27::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/angle.rs
//     evidence={"module":3807,"record_offset":8396,"parent_record_offset":8372,"ancestor_record_offset":272,"inlinee":16061,"source_line":11,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::background::impl$8::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/background.rs
//     evidence={"module":3807,"record_offset":8268,"parent_record_offset":8240,"ancestor_record_offset":272,"inlinee":16055,"source_line":5,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::color::impl$12::eq
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/color.rs
//     evidence={"module":3807,"record_offset":4132,"parent_record_offset":4108,"ancestor_record_offset":272,"inlinee":16025,"source_line":5,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::gradient::impl$21::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/gradient.rs
//     evidence={"module":3807,"record_offset":8372,"parent_record_offset":8348,"ancestor_record_offset":272,"inlinee":16059,"source_line":50,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::gradient::impl$8::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/gradient.rs
//     evidence={"module":3807,"record_offset":8324,"parent_record_offset":8300,"ancestor_record_offset":272,"inlinee":16057,"source_line":6,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$14::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":3807,"record_offset":7260,"parent_record_offset":7224,"ancestor_record_offset":272,"inlinee":16049,"source_line":9,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$19::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":3807,"record_offset":7292,"parent_record_offset":7260,"ancestor_record_offset":272,"inlinee":16051,"source_line":138,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$27::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":3807,"record_offset":9144,"parent_record_offset":9116,"ancestor_record_offset":272,"inlinee":16080,"source_line":170,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::pixels::impl$9::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/pixels.rs
//     evidence={"module":3807,"record_offset":4932,"parent_record_offset":4908,"ancestor_record_offset":272,"inlinee":16027,"source_line":9,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::point::impl$13::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/point.rs
//     evidence={"module":3807,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":272,"inlinee":16023,"source_line":7,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::rectangle::impl$9::eq
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/rectangle.rs
//     evidence={"module":3807,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":272,"inlinee":16010,"source_line":4,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::svg::impl$7::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/svg.rs
//     evidence={"module":3807,"record_offset":7904,"parent_record_offset":7876,"ancestor_record_offset":272,"inlinee":16053,"source_line":10,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::text::impl$14::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/text.rs
//     evidence={"module":3807,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":272,"inlinee":16035,"source_line":46,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::text::impl$23::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/text.rs
//     evidence={"module":3807,"record_offset":4988,"parent_record_offset":4964,"ancestor_record_offset":272,"inlinee":16029,"source_line":70,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::transformation::impl$14::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/transformation.rs
//     evidence={"module":3807,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":272,"inlinee":16006,"source_line":7,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_graphics::primitive::impl$4::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/graphics/src/primitive.rs
//     evidence={"module":3807,"record_offset":2808,"parent_record_offset":2760,"ancestor_record_offset":272,"inlinee":16015,"source_line":16,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::primitive::impl$4::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/primitive.rs
//     evidence={"module":3807,"record_offset":5624,"parent_record_offset":5592,"ancestor_record_offset":272,"inlinee":16037,"source_line":6,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":3807,"record_offset":7480,"parent_record_offset":7412,"ancestor_record_offset":272,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":3807,"record_offset":7452,"parent_record_offset":7412,"ancestor_record_offset":272,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":3807,"record_offset":7412,"parent_record_offset":7388,"ancestor_record_offset":272,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":3807,"record_offset":7388,"parent_record_offset":7364,"ancestor_record_offset":272,"inlinee":16069,"source_line":2218,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] strict_num::impl$2::eq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"strict-num","version":"0.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/strict-num-0.1.1/src/lib.rs
//     evidence={"module":3807,"record_offset":6196,"parent_record_offset":6172,"ancestor_record_offset":272,"inlinee":8346,"source_line":137,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia::blend_mode::impl$7::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/blend_mode.rs
//     evidence={"module":3807,"record_offset":6368,"parent_record_offset":6344,"ancestor_record_offset":272,"inlinee":16047,"source_line":4,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia::painter::impl$11::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/painter.rs
//     evidence={"module":3807,"record_offset":7184,"parent_record_offset":7160,"ancestor_record_offset":272,"inlinee":16067,"source_line":22,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia::painter::impl$15::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/painter.rs
//     evidence={"module":3807,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":272,"inlinee":16045,"source_line":37,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::impl$13::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/lib.rs
//     evidence={"module":3807,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":272,"inlinee":16041,"source_line":70,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::path::impl$15::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/path.rs
//     evidence={"module":3807,"record_offset":5700,"parent_record_offset":5676,"ancestor_record_offset":272,"inlinee":16039,"source_line":48,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::path::impl$9::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/path.rs
//     evidence={"module":3807,"record_offset":5908,"parent_record_offset":5884,"ancestor_record_offset":272,"inlinee":9228,"source_line":18,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   [pdb_inline_transitive] tiny_skia_path::rect::impl$15::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny-skia-path","version":"0.11.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-path-0.11.4/src/rect.rs
//     evidence={"module":3807,"record_offset":6172,"parent_record_offset":5700,"ancestor_record_offset":272,"inlinee":16043,"source_line":214,"archive":"_pdb/C/modules/03807.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::run_vulkan_configuration_gui
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":273,"record_offset":272,"rva":7104944,"va":5375814064}
// 编译实例 RVAs: 0x6c69b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140ff08b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9608879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375814064,"raw_hex":"e8af9e9200","size":5,"soft_fail":false,"target_va":5385423024,"va":5375814140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375814064,"raw_hex":"e8a2080000","size":5,"soft_fail":false,"target_va":5375816496,"va":5375814281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_gpu_selector::vulkan_config::Controls::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9596300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375814064,"raw_hex":"e88c6d9200","size":5,"soft_fail":false,"target_va":5385410416,"va":5375814111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":273,"record_offset":472,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00273.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":273,"record_offset":412,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00273.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":273,"record_offset":444,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":273,"record_offset":496,"parent_record_offset":472,"ancestor_record_offset":272,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":273,"record_offset":532,"parent_record_offset":496,"ancestor_record_offset":272,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":273,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":272,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":273,"record_offset":588,"parent_record_offset":496,"ancestor_record_offset":272,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00273.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::init::GraphicsLayerAllowlist,tg_gpu_selector::Workarounds>", "constructor": "tuple$", "arguments": ["rhapsody::raw::init::GraphicsLayerAllowlist", "tg_gpu_selector::Workarounds"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tuple$<rhapsody::raw::init::GraphicsLayerAllowlist,tg_gpu_selector::Workarounds> >", "constructor": "core::option::Option", "arguments": ["tuple$<rhapsody::raw::init::GraphicsLayerAllowlist,tg_gpu_selector::Workarounds>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsLayerAllowlist,tg_gpu_selector::Workarounds> > >", "constructor": "enum2$", "arguments": ["core::option::Option<tuple$<rhapsody::raw::init::GraphicsLayerAllowlist,tg_gpu_selector::Workarounds> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error>", "constructor": "core::result::Result", "arguments": ["enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsLayerAllowlist,tg_gpu_selector::Workarounds> > >", "anyhow::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)
// 候选参数类型: ["ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"]

// ------------------------------------------------------------------
// 函数: iced_graphics::damage::impl$0::bounds<enum2$<iced_tiny_skia::primitive::Custom> >
// 信息 ID: 15f53d325275b6ef70945805
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 089084bcf65bc8c4f0175768: iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// [类型关系] enum2$ -> ["iced_tiny_skia::primitive::Custom"]; 字段布局 = unknown。
// [类型关系] iced_graphics::damage::impl$0::bounds -> ["enum2$<iced_tiny_skia::primitive::Custom>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::chain::Chain<core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)> > > @ 0x1f59b31 -> 0x2837cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::rectangle::Rectangle<f32>,core::iter::adapters::chain::Chain<core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)> > > @ 0x1f598f5 -> 0x2837cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> > @ 0x28366e4 -> 0x2837cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> > @ 0x28366f1 -> 0x2837cd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: iced_graphics::damage::regions<enum2$<iced_tiny_skia::primitive::Custom> >
// 信息 ID: b3a2f2271618f3c24ef65d87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["iced_tiny_skia::primitive::Custom"]; 字段布局 = unknown。
// [类型关系] iced_graphics::damage::regions -> ["enum2$<iced_tiny_skia::primitive::Custom>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > > @ 0x1f57dfd -> 0x2835740 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >,core::slice::iter::Iter<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > > >,alloc::vec::Vec<iced_core::rectangle::Rectangle<f32>,alloc::alloc::Global>,iced_graphics::damage::list::closure_env$0<enum2$<iced_tiny_skia::primitive::Custom> > > @ 0x1f57f06 -> 0x2835740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::run_vulkan_configuration_gui
// 信息 ID: f7b347f30e843ec33f61d517
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::vulkan_config_main::run_vulkan_configuration_app @ 0x4bfbf6 -> 0x6c69b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
