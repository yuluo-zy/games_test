// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/water/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: edbe5dab9141024b36fda8bacd38ee5890e91555e352fd699c7041eefb47506e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 312, 1073, 2583, 2587, 3879, 3882, 3885, 5066
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"}]
// use std; // namespace candidate
//   package identities: []
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
// 首条位置: {"module":2587,"record_offset":6280,"rva":34058288,"va":5402767408}
// 编译实例 RVAs: 0x207b030, 0x285b4b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t13853856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402767408,"raw_hex":"e8a064d300","size":5,"soft_fail":false,"target_va":5416621762,"va":5402767901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":2587,"record_offset":6556,"parent_record_offset":6280,"ancestor_record_offset":6280,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/02587.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2587,"record_offset":6684,"parent_record_offset":6280,"ancestor_record_offset":6280,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/02587.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2587,"record_offset":6464,"parent_record_offset":6280,"ancestor_record_offset":6280,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/02587.jsonl.gz"}
//   [pdb_inline_direct] core::str::pattern::impl$36::matching
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2587,"record_offset":6836,"parent_record_offset":6280,"ancestor_record_offset":6280,"inlinee":9307,"source_line":1740,"archive":"_pdb/C/modules/02587.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2587,"record_offset":6640,"parent_record_offset":6612,"ancestor_record_offset":6280,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/02587.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2587,"record_offset":6772,"parent_record_offset":6712,"ancestor_record_offset":6280,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/02587.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2587,"record_offset":6712,"parent_record_offset":6684,"ancestor_record_offset":6280,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/02587.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2587,"record_offset":6612,"parent_record_offset":6584,"ancestor_record_offset":6280,"inlinee":9305,"source_line":813,"archive":"_pdb/C/modules/02587.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2587,"record_offset":6584,"parent_record_offset":6556,"ancestor_record_offset":6280,"inlinee":9304,"source_line":978,"archive":"_pdb/C/modules/02587.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2587,"record_offset":6796,"parent_record_offset":6772,"ancestor_record_offset":6280,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/02587.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2587,"record_offset":6516,"parent_record_offset":6464,"ancestor_record_offset":6280,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02587.jsonl.gz"}
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
// 首条位置: {"module":3885,"record_offset":3444,"rva":42320496,"va":5411029616}
// 编译实例 RVAs: 0x285c270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t5589913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411029616,"raw_hex":"e8994b5500","size":5,"soft_fail":false,"target_va":5416621762,"va":5411031844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::split
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-12356611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411029616,"raw_hex":"e8fd7343ff","size":5,"soft_fail":false,"target_va":5398673952,"va":5411030558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::split_at_max_curvature
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-12358570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411029616,"raw_hex":"e8566c43ff","size":5,"soft_fail":false,"target_va":5398671408,"va":5411029973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::to_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-12360674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411029616,"raw_hex":"e81e6443ff","size":5,"soft_fail":false,"target_va":5398671072,"va":5411031741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3885,"record_offset":3908,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] zeno::geometry::Vector::nearly_eq_by
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3885,"record_offset":5972,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":38463,"source_line":130,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Curve::is_line
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3885,"record_offset":3676,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":38461,"source_line":160,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Curve::needs_split
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3885,"record_offset":4016,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":38464,"source_line":306,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3885,"record_offset":3972,"parent_record_offset":3940,"ancestor_record_offset":3444,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3885,"record_offset":3748,"parent_record_offset":3716,"ancestor_record_offset":3444,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3885,"record_offset":4228,"parent_record_offset":4204,"ancestor_record_offset":3444,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3885,"record_offset":3940,"parent_record_offset":3908,"ancestor_record_offset":3444,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3885,"record_offset":4204,"parent_record_offset":4176,"ancestor_record_offset":3444,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] zeno::geometry::Vector::dot
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3885,"record_offset":4472,"parent_record_offset":4432,"ancestor_record_offset":3444,"inlinee":38471,"source_line":82,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] zeno::geometry::Vector::length
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3885,"record_offset":4176,"parent_record_offset":4136,"ancestor_record_offset":3444,"inlinee":38468,"source_line":64,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] zeno::geometry::Vector::nearly_eq_by
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3885,"record_offset":3716,"parent_record_offset":3676,"ancestor_record_offset":3444,"inlinee":38463,"source_line":130,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] zeno::geometry::Vector::normalize
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3885,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":3444,"inlinee":38467,"source_line":94,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] zeno::geometry::normal
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3885,"record_offset":4112,"parent_record_offset":4016,"ancestor_record_offset":3444,"inlinee":38466,"source_line":249,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] zeno::segment::impl$1::needs_split::too_curvy
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3885,"record_offset":4432,"parent_record_offset":4016,"ancestor_record_offset":3444,"inlinee":38470,"source_line":312,"archive":"_pdb/C/modules/03885.jsonl.gz"}
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
// 首条位置: {"module":3885,"record_offset":636,"rva":42318112,"va":5411027232}
// 编译实例 RVAs: 0x285b920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t5592297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411027232,"raw_hex":"e8e9545500","size":5,"soft_fail":false,"target_va":5416621762,"va":5411029460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::split
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-12354227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411027232,"raw_hex":"e84d7d43ff","size":5,"soft_fail":false,"target_va":5398673952,"va":5411028174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::split_at_max_curvature
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-12356186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411027232,"raw_hex":"e8a67543ff","size":5,"soft_fail":false,"target_va":5398671408,"va":5411027589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::to_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-12358290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411027232,"raw_hex":"e86e6d43ff","size":5,"soft_fail":false,"target_va":5398671072,"va":5411029357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3885,"record_offset":1312,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] zeno::geometry::Vector::nearly_eq_by
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3885,"record_offset":3376,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":38463,"source_line":130,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Curve::is_line
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3885,"record_offset":1080,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":38461,"source_line":160,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Curve::needs_split
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3885,"record_offset":1420,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":38464,"source_line":306,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3885,"record_offset":1376,"parent_record_offset":1344,"ancestor_record_offset":636,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3885,"record_offset":1152,"parent_record_offset":1120,"ancestor_record_offset":636,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3885,"record_offset":1632,"parent_record_offset":1608,"ancestor_record_offset":636,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3885,"record_offset":1344,"parent_record_offset":1312,"ancestor_record_offset":636,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3885,"record_offset":1608,"parent_record_offset":1580,"ancestor_record_offset":636,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] zeno::geometry::Vector::dot
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3885,"record_offset":1876,"parent_record_offset":1836,"ancestor_record_offset":636,"inlinee":38471,"source_line":82,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] zeno::geometry::Vector::length
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3885,"record_offset":1580,"parent_record_offset":1540,"ancestor_record_offset":636,"inlinee":38468,"source_line":64,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] zeno::geometry::Vector::nearly_eq_by
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3885,"record_offset":1120,"parent_record_offset":1080,"ancestor_record_offset":636,"inlinee":38463,"source_line":130,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] zeno::geometry::Vector::normalize
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3885,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":636,"inlinee":38467,"source_line":94,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] zeno::geometry::normal
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3885,"record_offset":1516,"parent_record_offset":1420,"ancestor_record_offset":636,"inlinee":38466,"source_line":249,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] zeno::segment::impl$1::needs_split::too_curvy
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3885,"record_offset":1836,"parent_record_offset":1420,"ancestor_record_offset":636,"inlinee":38470,"source_line":312,"archive":"_pdb/C/modules/03885.jsonl.gz"}
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
// 首条位置: {"module":3885,"record_offset":7784,"rva":42324624,"va":5411033744}
// 编译实例 RVAs: 0x285d290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5586435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411033744,"raw_hex":"e8033e5500","size":5,"soft_fail":false,"target_va":5416621762,"va":5411035322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::to_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-12363009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411033744,"raw_hex":"e8ff5a43ff","size":5,"soft_fail":false,"target_va":5398671072,"va":5411034076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Segments::split_curve<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > > >
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-4367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411033744,"raw_hex":"e8f1eeffff","size":5,"soft_fail":false,"target_va":5411029616,"va":5411033978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":3885,"record_offset":8020,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$47::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3885,"record_offset":8256,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3885,"record_offset":8724,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] zeno::geometry::Vector::nearly_eq_by
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3885,"record_offset":8284,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":38463,"source_line":130,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Curve::from_quadratic
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3885,"record_offset":8192,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":38506,"source_line":88,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Curve::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3885,"record_offset":8344,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":38501,"source_line":73,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Segments::inc_id
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3885,"record_offset":8228,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":38503,"source_line":494,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3885,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":7784,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3885,"record_offset":8056,"parent_record_offset":8020,"ancestor_record_offset":7784,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3885,"record_offset":8156,"parent_record_offset":8096,"ancestor_record_offset":7784,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3885,"record_offset":8128,"parent_record_offset":8096,"ancestor_record_offset":7784,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3885,"record_offset":8096,"parent_record_offset":8020,"ancestor_record_offset":7784,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zeno::segment::impl$5::next<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3885,"record_offset":6040,"rva":42322880,"va":5411032000}
// 编译实例 RVAs: 0x285cbc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5588023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411032000,"raw_hex":"e837445500","size":5,"soft_fail":false,"target_va":5416621762,"va":5411033734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::command::Command::transform
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-5482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411032000,"raw_hex":"e896eaffff","size":5,"soft_fail":false,"target_va":5411026928,"va":5411032405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Curve::to_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-12361233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411032000,"raw_hex":"e8ef6143ff","size":5,"soft_fail":false,"target_va":5398671072,"va":5411032300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::segment::Segments::split_curve<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > >
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t-4981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411032000,"raw_hex":"e88becffff","size":5,"soft_fail":false,"target_va":5411027232,"va":5411032208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3885,"record_offset":6516,"parent_record_offset":6040,"ancestor_record_offset":6040,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$47::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3885,"record_offset":6752,"parent_record_offset":6040,"ancestor_record_offset":6040,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3885,"record_offset":7336,"parent_record_offset":6040,"ancestor_record_offset":6040,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] zeno::geometry::Vector::nearly_eq_by
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/geometry.rs
//     evidence={"module":3885,"record_offset":6828,"parent_record_offset":6040,"ancestor_record_offset":6040,"inlinee":38463,"source_line":130,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Curve::from_quadratic
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3885,"record_offset":6792,"parent_record_offset":6040,"ancestor_record_offset":6040,"inlinee":38506,"source_line":88,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Curve::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3885,"record_offset":6460,"parent_record_offset":6040,"ancestor_record_offset":6040,"inlinee":38501,"source_line":73,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_direct] zeno::segment::Segments::inc_id
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/segment.rs
//     evidence={"module":3885,"record_offset":6488,"parent_record_offset":6040,"ancestor_record_offset":6040,"inlinee":38503,"source_line":494,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3885,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":6040,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":3885,"record_offset":6540,"parent_record_offset":6516,"ancestor_record_offset":6040,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3885,"record_offset":6688,"parent_record_offset":6660,"ancestor_record_offset":6040,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3885,"record_offset":6660,"parent_record_offset":6516,"ancestor_record_offset":6040,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3885,"record_offset":6624,"parent_record_offset":6564,"ancestor_record_offset":6040,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3885,"record_offset":6596,"parent_record_offset":6564,"ancestor_record_offset":6040,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3885,"record_offset":6564,"parent_record_offset":6540,"ancestor_record_offset":6040,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03885.jsonl.gz"}
//   [pdb_inline_transitive] zeno::scratch::impl$1::apply::closure$0
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/scratch.rs
//     evidence={"module":3885,"record_offset":6712,"parent_record_offset":6688,"ancestor_record_offset":6040,"inlinee":38505,"source_line":89,"archive":"_pdb/C/modules/03885.jsonl.gz"}
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
// 首条位置: {"module":312,"record_offset":1520,"rva":7256512,"va":5375965632}
// 编译实例 RVAs: 0x6eb9c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..boxed..Box$LT$dyn$u20$bevy_ecs..system..system..System$u2b$In$u20$$u3d$$u20$$LP$$RP$$u2b$Out$u20$$u3d$$u20$$LP$$RP$$GT$$u20$as$u20$bevy_ecs..schedule..config..Schedulable$GT$::into_config
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8252920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375965632,"raw_hex":"e8f8ed7d00","size":5,"soft_fail":false,"target_va":5384218864,"va":5375965939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$bevy_ecs..schedule..config..ScheduleConfigs$LT$T$GT$$u20$as$u20$bevy_ecs..schedule..config..IntoScheduleConfigs$LT$T$C$$LP$$RP$$GT$$GT$::run_if
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10017849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375965632,"raw_hex":"e839dc9800","size":5,"soft_fail":false,"target_va":5385983808,"va":5375965954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-287207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375965632,"raw_hex":"e8199efbff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375965890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7243688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375965632,"raw_hex":"e8587891ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375965875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400793a0
//     owner=unresolved_namespace; count=2
//     target_aliases=118 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6761280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375965632,"raw_hex":"e8c0d498ff","size":5,"soft_fail":false,"target_va":5369205664,"va":5375966939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5368107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375965632,"raw_hex":"e8d516aeff","size":5,"soft_fail":false,"target_va":5370597904,"va":5375966006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ff9f0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2178613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375965632,"raw_hex":"e8353e2100","size":5,"soft_fail":false,"target_va":5378144752,"va":5375966134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141077cc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10011338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375965632,"raw_hex":"e8cac29800","size":5,"soft_fail":false,"target_va":5385977024,"va":5375965681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410785b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10013604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375965632,"raw_hex":"e8a4cb9800","size":5,"soft_fail":false,"target_va":5385979312,"va":5375965703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40653867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375965632,"raw_hex":"e82b546c02","size":5,"soft_fail":false,"target_va":5416620769,"va":5375966897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10020842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375965632,"raw_hex":"e8eae79800","size":5,"soft_fail":false,"target_va":5385986992,"va":5375966145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10013725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375965632,"raw_hex":"e81dcc9800","size":5,"soft_fail":false,"target_va":5385979600,"va":5375965870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10018376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375965632,"raw_hex":"e848de9800","size":5,"soft_fail":false,"target_va":5385984368,"va":5375965987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::message::message_registry::MessageRegistry::register_message
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10023254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375965632,"raw_hex":"e856f19800","size":5,"soft_fail":false,"target_va":5385989648,"va":5375966389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::world_raster::DirtyTilePublisher::unique
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5684692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375965632,"raw_hex":"e8d4bd5600","size":5,"soft_fail":false,"target_va":5381650816,"va":5375966119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_message
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":312,"record_offset":2036,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":11549,"source_line":400,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":312,"record_offset":1644,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":312,"record_offset":1672,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":4638,"source_line":448,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":312,"record_offset":1936,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":5917,"source_line":493,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":312,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":1520,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":312,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":1520,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":312,"record_offset":1876,"parent_record_offset":1852,"ancestor_record_offset":1520,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":312,"record_offset":1804,"parent_record_offset":1772,"ancestor_record_offset":1520,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":312,"record_offset":1772,"parent_record_offset":1736,"ancestor_record_offset":1520,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":312,"record_offset":1736,"parent_record_offset":1700,"ancestor_record_offset":1520,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":312,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":1520,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":312,"record_offset":3480,"parent_record_offset":3456,"ancestor_record_offset":1520,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":312,"record_offset":3456,"parent_record_offset":3432,"ancestor_record_offset":1520,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":312,"record_offset":3408,"parent_record_offset":3384,"ancestor_record_offset":1520,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":312,"record_offset":3384,"parent_record_offset":3344,"ancestor_record_offset":1520,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":312,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":1520,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_message
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs
//     evidence={"module":312,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":1520,"inlinee":9392,"source_line":348,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":312,"record_offset":2120,"parent_record_offset":2088,"ancestor_record_offset":1520,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::impl$4::into_configs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":312,"record_offset":1700,"parent_record_offset":1672,"ancestor_record_offset":1520,"inlinee":4259,"source_line":565,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/label.rs
//     evidence={"module":312,"record_offset":1976,"parent_record_offset":1936,"ancestor_record_offset":1520,"inlinee":4539,"source_line":119,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":312,"record_offset":3652,"parent_record_offset":3628,"ancestor_record_offset":1520,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":312,"record_offset":3628,"parent_record_offset":3604,"ancestor_record_offset":1520,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":312,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":1520,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":312,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":1520,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":312,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":1520,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":312,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":1520,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":312,"record_offset":2088,"parent_record_offset":2060,"ancestor_record_offset":1520,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":312,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":1520,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":312,"record_offset":2144,"parent_record_offset":2120,"ancestor_record_offset":1520,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":312,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":1520,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":312,"record_offset":2820,"parent_record_offset":2792,"ancestor_record_offset":1520,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":312,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":1520,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":312,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":1520,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":312,"record_offset":2592,"parent_record_offset":2564,"ancestor_record_offset":1520,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":312,"record_offset":2564,"parent_record_offset":2536,"ancestor_record_offset":1520,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":312,"record_offset":3232,"parent_record_offset":3168,"ancestor_record_offset":1520,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":312,"record_offset":3044,"parent_record_offset":2088,"ancestor_record_offset":1520,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":312,"record_offset":3256,"parent_record_offset":3232,"ancestor_record_offset":1520,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":312,"record_offset":3580,"parent_record_offset":2088,"ancestor_record_offset":1520,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":312,"record_offset":3296,"parent_record_offset":3116,"ancestor_record_offset":1520,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":312,"record_offset":2384,"parent_record_offset":2360,"ancestor_record_offset":1520,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":312,"record_offset":2708,"parent_record_offset":2684,"ancestor_record_offset":1520,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":312,"record_offset":3168,"parent_record_offset":3140,"ancestor_record_offset":1520,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":312,"record_offset":3204,"parent_record_offset":3168,"ancestor_record_offset":1520,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":312,"record_offset":3524,"parent_record_offset":3344,"ancestor_record_offset":1520,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":312,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":1520,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":312,"record_offset":2504,"parent_record_offset":2476,"ancestor_record_offset":1520,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":312,"record_offset":2536,"parent_record_offset":2504,"ancestor_record_offset":1520,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":312,"record_offset":2476,"parent_record_offset":2256,"ancestor_record_offset":1520,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":312,"record_offset":2336,"parent_record_offset":2256,"ancestor_record_offset":1520,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":312,"record_offset":2896,"parent_record_offset":2256,"ancestor_record_offset":1520,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":312,"record_offset":2420,"parent_record_offset":2256,"ancestor_record_offset":1520,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":312,"record_offset":2180,"parent_record_offset":2144,"ancestor_record_offset":1520,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":312,"record_offset":2744,"parent_record_offset":2636,"ancestor_record_offset":1520,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":312,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":1520,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":312,"record_offset":2988,"parent_record_offset":2256,"ancestor_record_offset":1520,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":312,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":1520,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":312,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":1520,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":312,"record_offset":2208,"parent_record_offset":2180,"ancestor_record_offset":1520,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":312,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":1520,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":312,"record_offset":2308,"parent_record_offset":2256,"ancestor_record_offset":1520,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":312,"record_offset":2636,"parent_record_offset":2256,"ancestor_record_offset":1520,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00312.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":312,"record_offset":2868,"parent_record_offset":2256,"ancestor_record_offset":1520,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00312.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$15::add_to_app<bevy_app::plugin::sealed::PluginMarker,feedback_reporter::FeedbackReporterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::menu_background::MenuBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::paths::PathPlugin,bevy_app::plugin::sealed::PluginMarker,system_water::WaterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::garden::GardenPlugin,bevy_app::plugin::sealed::PluginMarker,system_tree::TreePlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::prefab::PrefabPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest,bevy_app::plugin::sealed::PluginMarker,country_core::resources::photo_mode::PhotoModePlugin>", "constructor": "bevy_app::plugin::sealed::impl$15::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "feedback_reporter::FeedbackReporterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::menu_background::MenuBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::paths::PathPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_water::WaterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::garden::GardenPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_tree::TreePlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::prefab::PrefabPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::vermeer::VermeerPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::photo_mode::PhotoModePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<system_water::WaterPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["system_water::WaterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<system_water::WaterPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["system_water::WaterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<system_water::WaterPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["system_water::WaterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<system_water::WaterPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["system_water::WaterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<system_water::WaterPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["system_water::WaterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterEraseEvent>,bevy_ecs::message::message_writer::MessageWriter<system_water::record_water_history::RecordWaterEditToHistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser> >,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "country_core::systems::world_raster::RasterDrawResources", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterEraseEvent>", "bevy_ecs::message::message_writer::MessageWriter<system_water::record_water_history::RecordWaterEditToHistoryCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>", "bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser> >", "bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterEraseEvent>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_water::record_water_history::RecordWaterEditToHistoryCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::system_param::Local<system_water::water_audio_grid::WaterAudioGridUpdateState>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>)
// 候选参数类型: ["bevy_ecs::system::system_param::Local<system_water::water_audio_grid::WaterAudioGridUpdateState>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "country_core::systems::collision::world::Raycasting", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>"]
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_water::water_audio_grid::WaterAudioGridUpdateState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::message::message_reader::MessageReader<system_water::record_water_history::RecordWaterEditToHistoryCmd>)
// 候选参数类型: ["country_core::systems::world_raster::RasterHistoryRecordResources", "bevy_ecs::message::message_reader::MessageReader<system_water::record_water_history::RecordWaterEditToHistoryCmd>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_water::record_water_history::RecordWaterEditToHistoryCmd>

// ------------------------------------------------------------------
// 函数: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 信息 ID: 0538cde337678dd7ea2e895d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["core::str::pattern::MatchOnly"]; 字段布局 = unknown。
// [类型关系] core::str::pattern::TwoWaySearcher::next_back -> ["enum2$<core::str::pattern::MatchOnly>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x207af07 -> 0x207b030 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x285b383 -> 0x285b4b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_water::impl$0::build
// 信息 ID: 3f440d88ff92bffea2768515
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zeno::segment::Segments::split_curve<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > > >
// 信息 ID: e417aaa56fdac23f49271c5c
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
// 信息 ID: 0277a6ea68129cdf937ee286
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
// 信息 ID: a22643f185258cf88b374a8b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["zeno::command::Command"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["enum2$<zeno::command::Command>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::copied::Copied -> ["core::slice::iter::Iter<enum2$<zeno::command::Command> >"]; 字段布局 = unknown。
// [类型关系] zeno::segment::impl$5::next -> ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 20 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x2d36476 -> 0x285d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x2d381f0 -> 0x285d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > @ 0x2d3d7e8 -> 0x285d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > > @ 0x2d3f5e4 -> 0x285d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::geometry::BoundsBuilder> > @ 0x2d3fea7 -> 0x285d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::geometry::BoundsBuilder> > @ 0x2d40690 -> 0x285d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::geometry::BoundsBuilder> @ 0x2d40dc7 -> 0x285d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::geometry::BoundsBuilder> @ 0x2d415b0 -> 0x285d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::HeapStorage> >,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x2d43e3e -> 0x285d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::HeapStorage> >,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x2d459da -> 0x285d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::raster::Rasterizer<zeno::raster::HeapStorage> >,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x2d45af0 -> 0x285d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x2d45ddd -> 0x285d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x2d477cd -> 0x285d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x2d478e0 -> 0x285d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::geometry::BoundsBuilder,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x2d47bc0 -> 0x285d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::geometry::BoundsBuilder,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x2d482bd -> 0x285d290 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zeno::segment::impl$5::next<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > > >
// 信息 ID: d735b3cbcc34ddaadbacd4a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["zeno::command::Command"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["enum2$<zeno::command::Command>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::copied::Copied -> ["core::slice::iter::Iter<enum2$<zeno::command::Command> >"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["zeno::command::Command"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["enum2$<zeno::command::Command>"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<enum2$<zeno::command::Command> >"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["ref$<slice2$<enum2$<zeno::command::Command> > >"]; 字段布局 = unknown。
// [类型关系] zeno::raster::Rasterizer -> ["zeno::raster::HeapStorage"]; 字段布局 = unknown。
// [类型关系] zeno::scratch::impl$1::apply::closure_env$0 -> ["ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >", "zeno::raster::Rasterizer<zeno::raster::HeapStorage>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >", "zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> >"]; 字段布局 = unknown。
// [类型关系] zeno::segment::impl$5::next -> ["core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x2d38b86 -> 0x285cbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > >,zeno::raster::Rasterizer<zeno::raster::AdaptiveStorage> > @ 0x2d3a900 -> 0x285cbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::geometry::BoundsBuilder> >,zeno::geometry::BoundsBuilder> @ 0x2d3b296 -> 0x285cbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_into<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_data::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,enum2$<zeno::style::Style>,zeno::geometry::BoundsBuilder> >,zeno::geometry::BoundsBuilder> @ 0x2d3cef0 -> 0x285cbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::geometry::BoundsBuilder> >,zeno::geometry::BoundsBuilder,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x2d41cd0 -> 0x285cbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::geometry::BoundsBuilder> >,zeno::geometry::BoundsBuilder,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x2d43a3d -> 0x285cbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::geometry::BoundsBuilder> >,zeno::geometry::BoundsBuilder,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x2d43b60 -> 0x285cbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x2d490e0 -> 0x285cbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x2d4aad5 -> 0x285cbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x2d4abe0 -> 0x285cbc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
