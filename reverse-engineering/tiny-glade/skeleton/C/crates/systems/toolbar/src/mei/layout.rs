// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/mei/layout.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 3f8618f1b925157e659df9fde4c5404bb28d11f5248f563d3fbcbaa8cbfd57b9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: layout.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 90, 177, 196, 348, 355, 356, 1235, 1271, 2887, 2930, 4207
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1271,"record_offset":11988,"rva":18627456,"va":5387336576}
// 编译实例 RVAs: 0x11c3b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13857687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387336576,"raw_hex":"e8698c2cff","size":5,"soft_fail":false,"target_va":5373479040,"va":5387336722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29284255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387336576,"raw_hex":"e89fd7be01","size":5,"soft_fail":false,"target_va":5416621536,"va":5387337276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1271,"record_offset":13624,"parent_record_offset":11988,"ancestor_record_offset":11988,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1271,"record_offset":12368,"parent_record_offset":11988,"ancestor_record_offset":11988,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1271,"record_offset":15148,"parent_record_offset":11988,"ancestor_record_offset":11988,"inlinee":38823,"source_line":174,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1271,"record_offset":15320,"parent_record_offset":11988,"ancestor_record_offset":11988,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1271,"record_offset":16008,"parent_record_offset":11988,"ancestor_record_offset":11988,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1271,"record_offset":15920,"parent_record_offset":11988,"ancestor_record_offset":11988,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points_or_zero
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":15120,"parent_record_offset":11988,"ancestor_record_offset":11988,"inlinee":22120,"source_line":397,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::points_or_zero
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":15540,"parent_record_offset":11988,"ancestor_record_offset":11988,"inlinee":22148,"source_line":423,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::size_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":15776,"parent_record_offset":11988,"ancestor_record_offset":11988,"inlinee":22145,"source_line":116,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Size::points_or_zero
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":15348,"parent_record_offset":11988,"ancestor_record_offset":11988,"inlinee":22118,"source_line":413,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1271,"record_offset":13364,"parent_record_offset":13340,"ancestor_record_offset":11988,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1271,"record_offset":12932,"parent_record_offset":12908,"ancestor_record_offset":11988,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1271,"record_offset":13016,"parent_record_offset":12992,"ancestor_record_offset":11988,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1271,"record_offset":13432,"parent_record_offset":12796,"ancestor_record_offset":11988,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1271,"record_offset":12400,"parent_record_offset":12368,"ancestor_record_offset":11988,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$3::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1271,"record_offset":15172,"parent_record_offset":15148,"ancestor_record_offset":11988,"inlinee":70693,"source_line":149,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1271,"record_offset":13164,"parent_record_offset":13136,"ancestor_record_offset":11988,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1271,"record_offset":13136,"parent_record_offset":13108,"ancestor_record_offset":11988,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1271,"record_offset":15828,"parent_record_offset":15804,"ancestor_record_offset":11988,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1271,"record_offset":12596,"parent_record_offset":12572,"ancestor_record_offset":11988,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1271,"record_offset":12540,"parent_record_offset":12400,"ancestor_record_offset":11988,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1271,"record_offset":15072,"parent_record_offset":15048,"ancestor_record_offset":11988,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1271,"record_offset":12572,"parent_record_offset":12368,"ancestor_record_offset":11988,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1271,"record_offset":15272,"parent_record_offset":15248,"ancestor_record_offset":11988,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1271,"record_offset":12956,"parent_record_offset":12932,"ancestor_record_offset":11988,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1271,"record_offset":15248,"parent_record_offset":15224,"ancestor_record_offset":11988,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1271,"record_offset":13280,"parent_record_offset":13256,"ancestor_record_offset":11988,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1271,"record_offset":12508,"parent_record_offset":12432,"ancestor_record_offset":11988,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1271,"record_offset":15224,"parent_record_offset":15196,"ancestor_record_offset":11988,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1271,"record_offset":12472,"parent_record_offset":12432,"ancestor_record_offset":11988,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1271,"record_offset":15196,"parent_record_offset":15172,"ancestor_record_offset":11988,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1271,"record_offset":12432,"parent_record_offset":12400,"ancestor_record_offset":11988,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1271,"record_offset":13388,"parent_record_offset":13364,"ancestor_record_offset":11988,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1271,"record_offset":13076,"parent_record_offset":13048,"ancestor_record_offset":11988,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1271,"record_offset":13108,"parent_record_offset":13076,"ancestor_record_offset":11988,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1271,"record_offset":13048,"parent_record_offset":12796,"ancestor_record_offset":11988,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1271,"record_offset":12908,"parent_record_offset":12796,"ancestor_record_offset":11988,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1271,"record_offset":13460,"parent_record_offset":12796,"ancestor_record_offset":11988,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1271,"record_offset":12992,"parent_record_offset":12796,"ancestor_record_offset":11988,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1271,"record_offset":12852,"parent_record_offset":12796,"ancestor_record_offset":11988,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1271,"record_offset":13340,"parent_record_offset":13316,"ancestor_record_offset":11988,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1271,"record_offset":12692,"parent_record_offset":12668,"ancestor_record_offset":11988,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1271,"record_offset":13316,"parent_record_offset":13208,"ancestor_record_offset":11988,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1271,"record_offset":12720,"parent_record_offset":12692,"ancestor_record_offset":11988,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1271,"record_offset":13256,"parent_record_offset":13232,"ancestor_record_offset":11988,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1271,"record_offset":13552,"parent_record_offset":12796,"ancestor_record_offset":11988,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1271,"record_offset":13232,"parent_record_offset":13208,"ancestor_record_offset":11988,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1271,"record_offset":12772,"parent_record_offset":12748,"ancestor_record_offset":11988,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1271,"record_offset":12748,"parent_record_offset":12692,"ancestor_record_offset":11988,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1271,"record_offset":12796,"parent_record_offset":12772,"ancestor_record_offset":11988,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1271,"record_offset":12880,"parent_record_offset":12796,"ancestor_record_offset":11988,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1271,"record_offset":13208,"parent_record_offset":12796,"ancestor_record_offset":11988,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1271,"record_offset":12668,"parent_record_offset":12644,"ancestor_record_offset":11988,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1271,"record_offset":12644,"parent_record_offset":12620,"ancestor_record_offset":11988,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$17::perform_layout::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1271,"record_offset":12620,"parent_record_offset":12596,"ancestor_record_offset":11988,"inlinee":70691,"source_line":973,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentage::points_or_zero
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":15376,"parent_record_offset":15348,"ancestor_record_offset":11988,"inlinee":22120,"source_line":397,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::Rect::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":15804,"parent_record_offset":15776,"ancestor_record_offset":11988,"inlinee":22117,"source_line":85,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::Rect::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":15860,"parent_record_offset":15776,"ancestor_record_offset":11988,"inlinee":22116,"source_line":81,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::Size::points_or_zero
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":15580,"parent_record_offset":15540,"ancestor_record_offset":11988,"inlinee":22118,"source_line":413,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1271,"record_offset":17576,"rva":18629952,"va":5387339072}
// 编译实例 RVAs: 0x11c4540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142335640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18288854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387339072,"raw_hex":"e8d6101701","size":5,"soft_fail":false,"target_va":5405627968,"va":5387339109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14238a780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18637274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387339072,"raw_hex":"e8da611c01","size":5,"soft_fail":false,"target_va":5405976448,"va":5387339169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14238a880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18637254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387339072,"raw_hex":"e8c6611c01","size":5,"soft_fail":false,"target_va":5405976704,"va":5387339445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d71ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29021268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387339072,"raw_hex":"e854d4ba01","size":5,"soft_fail":false,"target_va":5416360864,"va":5387339591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387339072,"raw_hex":"e816e7ffff","size":5,"soft_fail":false,"target_va":5387333024,"va":5387339397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-6254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387339072,"raw_hex":"e892e7ffff","size":5,"soft_fail":false,"target_va":5387333216,"va":5387339465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1271,"record_offset":18108,"parent_record_offset":17576,"ancestor_record_offset":17576,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":17744,"parent_record_offset":17576,"ancestor_record_offset":17576,"inlinee":70695,"source_line":728,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1271,"record_offset":17932,"parent_record_offset":17900,"ancestor_record_offset":17576,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1271,"record_offset":17984,"parent_record_offset":17744,"ancestor_record_offset":17576,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1271,"record_offset":17900,"parent_record_offset":17744,"ancestor_record_offset":17576,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1271,"record_offset":17836,"parent_record_offset":17744,"ancestor_record_offset":17576,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1271,"record_offset":17780,"parent_record_offset":17744,"ancestor_record_offset":17576,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1271,"record_offset":17804,"parent_record_offset":17780,"ancestor_record_offset":17576,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1271,"record_offset":17872,"parent_record_offset":17744,"ancestor_record_offset":17576,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentageDeser::Naked::constructor$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":17952,"parent_record_offset":17932,"ancestor_record_offset":17576,"inlinee":70699,"source_line":731,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":18012,"parent_record_offset":17744,"ancestor_record_offset":17576,"inlinee":70695,"source_line":728,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::impl$26::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":18172,"parent_record_offset":18148,"ancestor_record_offset":17576,"inlinee":22124,"source_line":742,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize<serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1271,"record_offset":16916,"rva":18629424,"va":5387338544}
// 编译实例 RVAs: 0x11c4330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14238a780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18637830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387338544,"raw_hex":"e806641c01","size":5,"soft_fail":false,"target_va":5405976448,"va":5387338613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14238a880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18637957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387338544,"raw_hex":"e885641c01","size":5,"soft_fail":false,"target_va":5405976704,"va":5387338742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d71ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29021974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387338544,"raw_hex":"e816d7ba01","size":5,"soft_fail":false,"target_va":5416360864,"va":5387338885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387338544,"raw_hex":"e8d5e9ffff","size":5,"soft_fail":false,"target_va":5387333024,"va":5387338694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-5551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387338544,"raw_hex":"e851eaffff","size":5,"soft_fail":false,"target_va":5387333216,"va":5387338762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde::private::de::content::content_clone
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2721303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387338544,"raw_hex":"e8e979d6ff","size":5,"soft_fail":false,"target_va":5384617280,"va":5387338578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1271,"record_offset":17480,"parent_record_offset":16916,"ancestor_record_offset":16916,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":17132,"parent_record_offset":16916,"ancestor_record_offset":16916,"inlinee":70695,"source_line":728,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1271,"record_offset":17304,"parent_record_offset":17276,"ancestor_record_offset":16916,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1271,"record_offset":17356,"parent_record_offset":17132,"ancestor_record_offset":16916,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1271,"record_offset":17276,"parent_record_offset":17132,"ancestor_record_offset":16916,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1271,"record_offset":17220,"parent_record_offset":17132,"ancestor_record_offset":16916,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1271,"record_offset":17164,"parent_record_offset":17132,"ancestor_record_offset":16916,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::__deserialize_content_v1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1271,"record_offset":17188,"parent_record_offset":17164,"ancestor_record_offset":16916,"inlinee":38818,"source_line":2433,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1271,"record_offset":17248,"parent_record_offset":17132,"ancestor_record_offset":16916,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentageDeser::Naked::constructor$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":17324,"parent_record_offset":17304,"ancestor_record_offset":16916,"inlinee":70699,"source_line":731,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":17384,"parent_record_offset":17132,"ancestor_record_offset":16916,"inlinee":70695,"source_line":728,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::impl$26::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":17536,"parent_record_offset":17512,"ancestor_record_offset":16916,"inlinee":22124,"source_line":742,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Size::new
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2CB7
// 首条位置: {"module":90,"inlinee":11447}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1271,"record_offset":16304,"rva":18628624,"va":5387337744}
// 编译实例 RVAs: 0x11c4010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411c4330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387337744,"raw_hex":"e8a1020000","size":5,"soft_fail":false,"target_va":5387338544,"va":5387337866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142335640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18290177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387337744,"raw_hex":"e801161701","size":5,"soft_fail":false,"target_va":5405627968,"va":5387337786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14238a910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18638740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387337744,"raw_hex":"e894671c01","size":5,"soft_fail":false,"target_va":5405976848,"va":5387338103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d71ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29022505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387337744,"raw_hex":"e829d9ba01","size":5,"soft_fail":false,"target_va":5416360864,"va":5387338354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387337744,"raw_hex":"e838ecffff","size":5,"soft_fail":false,"target_va":5387333024,"va":5387338083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-5010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387337744,"raw_hex":"e86eecffff","size":5,"soft_fail":false,"target_va":5387333216,"va":5387338221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1271,"record_offset":16808,"parent_record_offset":16304,"ancestor_record_offset":16304,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":16520,"parent_record_offset":16304,"ancestor_record_offset":16304,"inlinee":70695,"source_line":728,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1271,"record_offset":16852,"parent_record_offset":16808,"ancestor_record_offset":16304,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1271,"record_offset":16684,"parent_record_offset":16520,"ancestor_record_offset":16304,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1271,"record_offset":16648,"parent_record_offset":16520,"ancestor_record_offset":16304,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1271,"record_offset":16612,"parent_record_offset":16520,"ancestor_record_offset":16304,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1271,"record_offset":16556,"parent_record_offset":16520,"ancestor_record_offset":16304,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1271,"record_offset":16580,"parent_record_offset":16556,"ancestor_record_offset":16304,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$190::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1271,"record_offset":16712,"parent_record_offset":16520,"ancestor_record_offset":16304,"inlinee":7640,"source_line":1399,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::impl$27::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":16876,"parent_record_offset":16852,"ancestor_record_offset":16304,"inlinee":70697,"source_line":767,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Rect::width
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5664
// 首条位置: {"module":355,"inlinee":22116}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1271,"record_offset":10192,"parent_record_offset":10168,"ancestor_record_offset":10168,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Rect::height
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5665
// 首条位置: {"module":355,"inlinee":22117}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1271,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":7540,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Rect::size_vec2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5681
// 首条位置: {"module":1271,"inlinee":22145}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::Rect::height
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":7540,"parent_record_offset":7516,"ancestor_record_offset":7516,"inlinee":22117,"source_line":85,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::width
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":10168,"parent_record_offset":10144,"ancestor_record_offset":10144,"inlinee":22116,"source_line":81,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1271,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":7516,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::impl$11::from_points
// 记录形式: inlinee_declaration; instances=1
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1142F
// 首条位置: {"module":1271,"inlinee":70703}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::impl$13::from_points
// 记录形式: inlinee_declaration; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1142D
// 首条位置: {"module":1271,"inlinee":70701}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Axis::is_horizontal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55C9
// 首条位置: {"module":348,"inlinee":21961}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 227, 237, 728, 735, 753
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11427
// 首条位置: {"module":1271,"inlinee":70695}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1271,"record_offset":16684,"parent_record_offset":16520,"ancestor_record_offset":16520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1271,"record_offset":16648,"parent_record_offset":16520,"ancestor_record_offset":16520,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1271,"record_offset":16612,"parent_record_offset":16520,"ancestor_record_offset":16520,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1271,"record_offset":16556,"parent_record_offset":16520,"ancestor_record_offset":16520,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impls::impl$190::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1271,"record_offset":16712,"parent_record_offset":16520,"ancestor_record_offset":16520,"inlinee":7640,"source_line":1399,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1271,"record_offset":17248,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":17384,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":70695,"source_line":728,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1271,"record_offset":17304,"parent_record_offset":17276,"ancestor_record_offset":17132,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::__deserialize_content_v1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1271,"record_offset":17188,"parent_record_offset":17164,"ancestor_record_offset":17132,"inlinee":38818,"source_line":2433,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1271,"record_offset":16580,"parent_record_offset":16556,"ancestor_record_offset":16520,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentageDeser::Naked::constructor$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":17324,"parent_record_offset":17304,"ancestor_record_offset":17132,"inlinee":70699,"source_line":731,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 227, 237
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1271,"record_offset":264,"rva":18622672,"va":5387331792}
// 编译实例 RVAs: 0x11c28d0, 0x11c29f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t29223734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387331792,"raw_hex":"e836ebbd01","size":5,"soft_fail":false,"target_va":5416555792,"va":5387332053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1271,"record_offset":440,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1271,"record_offset":464,"parent_record_offset":440,"ancestor_record_offset":264,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1271,"record_offset":488,"parent_record_offset":464,"ancestor_record_offset":264,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1271,"record_offset":516,"parent_record_offset":488,"ancestor_record_offset":264,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 227, 237, 735
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B48E
// 首条位置: {"module":2887,"inlinee":111758}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":13896,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":9935,"source_line":759,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":2930,"record_offset":576,"parent_record_offset":544,"ancestor_record_offset":544,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2930,"record_offset":900,"parent_record_offset":876,"ancestor_record_offset":544,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2930,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":544,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2930,"record_offset":924,"parent_record_offset":900,"ancestor_record_offset":544,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":2930,"record_offset":1048,"parent_record_offset":1024,"ancestor_record_offset":544,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":2930,"record_offset":1024,"parent_record_offset":576,"ancestor_record_offset":544,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2930,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":544,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2930,"record_offset":852,"parent_record_offset":828,"ancestor_record_offset":544,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2930,"record_offset":828,"parent_record_offset":576,"ancestor_record_offset":544,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2930,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":544,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":13932,"parent_record_offset":13896,"ancestor_record_offset":13872,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":13996,"parent_record_offset":13896,"ancestor_record_offset":13872,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2930,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":544,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2930,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":544,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2887,"record_offset":13968,"parent_record_offset":13896,"ancestor_record_offset":13872,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2930,"record_offset":636,"parent_record_offset":576,"ancestor_record_offset":544,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":2930,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":544,"inlinee":70705,"source_line":735,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":2930,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":544,"inlinee":70690,"source_line":735,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 227, 237
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11430
// 首条位置: {"module":2887,"inlinee":70704}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":77896,"parent_record_offset":77844,"ancestor_record_offset":77844,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2887,"record_offset":77868,"parent_record_offset":77844,"ancestor_record_offset":77844,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::impl$16::default
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 283
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5672
// 首条位置: {"module":355,"record_offset":264,"rva":7544352,"va":5376253472}
// 编译实例 RVAs: 0x731e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::vertical
// 记录形式: compiled_procedure; instances=1
// 源码行: 298
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":355,"record_offset":1900,"rva":7544992,"va":5376254112}
// 编译实例 RVAs: 0x7320a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::LayoutStyle::vertical
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":355,"record_offset":2016,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":22122,"source_line":312,"archive":"_pdb/C/modules/00355.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::horizontal
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 302
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5673
// 首条位置: {"module":355,"record_offset":380,"rva":7544400,"va":5376253520}
// 编译实例 RVAs: 0x731e50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::LayoutStyle::horizontal
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":355,"record_offset":496,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":22114,"source_line":307,"archive":"_pdb/C/modules/00355.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::horizontal
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 307
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5662
// 首条位置: {"module":355,"inlinee":22114}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::vertical
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 312
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x566A
// 首条位置: {"module":355,"inlinee":22122}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::gap
// 记录形式: compiled_procedure; instances=1
// 源码行: 317
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":355,"record_offset":1020,"rva":7544656,"va":5376253776}
// 编译实例 RVAs: 0x731f50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::padding
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 322
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x567F
// 首条位置: {"module":355,"record_offset":1372,"rva":7544848,"va":5376253968}
// 编译实例 RVAs: 0x732010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::expand
// 记录形式: compiled_procedure; instances=1
// 源码行: 327
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":355,"record_offset":1252,"rva":7544784,"va":5376253904}
// 编译实例 RVAs: 0x731fd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::space_between
// 记录形式: compiled_procedure; instances=1
// 源码行: 332
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":355,"record_offset":772,"rva":7544560,"va":5376253680}
// 编译实例 RVAs: 0x731ef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::align_center
// 记录形式: compiled_procedure; instances=1
// 源码行: 342
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":355,"record_offset":524,"rva":7544448,"va":5376253568}
// 编译实例 RVAs: 0x731e80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::align_end
// 记录形式: compiled_procedure; instances=1
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":355,"record_offset":1492,"rva":7544912,"va":5376254032}
// 编译实例 RVAs: 0x732050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::xalign_center
// 记录形式: compiled_procedure; instances=1
// 源码行: 359
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":355,"record_offset":896,"rva":7544608,"va":5376253728}
// 编译实例 RVAs: 0x731f20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::size
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 371
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x567D
// 首条位置: {"module":355,"record_offset":1136,"rva":7544720,"va":5376253840}
// 编译实例 RVAs: 0x731f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::ComputedLayout::center
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 391
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55C7
// 首条位置: {"module":348,"inlinee":21959}
// 编译实例 RVAs: 0x732080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":348,"record_offset":68472,"parent_record_offset":68448,"ancestor_record_offset":68448,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00348.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":348,"record_offset":68528,"parent_record_offset":68448,"ancestor_record_offset":68448,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00348.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":348,"record_offset":68552,"parent_record_offset":68528,"ancestor_record_offset":68448,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00348.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":348,"record_offset":68496,"parent_record_offset":68472,"ancestor_record_offset":68448,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00348.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LengthPercentage::points_or_zero
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 397
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5668
// 首条位置: {"module":355,"inlinee":22120}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LengthPercentage::points
// 记录形式: inline_instance, inlinee_declaration; instances=24
// 源码行: 404
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5669
// 首条位置: {"module":355,"inlinee":22121}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Size::points_or_zero
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 413
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5666
// 首条位置: {"module":355,"inlinee":22118}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points_or_zero
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":15376,"parent_record_offset":15348,"ancestor_record_offset":15348,"inlinee":22120,"source_line":397,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Size::points
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 417
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5686
// 首条位置: {"module":1271,"inlinee":22150}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":8024,"parent_record_offset":8000,"ancestor_record_offset":8000,"inlinee":22121,"source_line":404,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Rect::points_or_zero
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 423
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5684
// 首条位置: {"module":1271,"inlinee":22148}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::Size::points_or_zero
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":15580,"parent_record_offset":15540,"ancestor_record_offset":15540,"inlinee":22118,"source_line":413,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentage::points_or_zero
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":15608,"parent_record_offset":15580,"ancestor_record_offset":15540,"inlinee":22120,"source_line":397,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Rect::points
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 438
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5685
// 首条位置: {"module":1271,"inlinee":22149}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":7384,"parent_record_offset":7332,"ancestor_record_offset":7332,"inlinee":22121,"source_line":404,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::place_childen
// 记录形式: compiled_procedure; instances=1
// 源码行: 497
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":355,"record_offset":648,"rva":7544496,"va":5376253616}
// 编译实例 RVAs: 0x731eb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411c3050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11080064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376253616,"raw_hex":"e88011a900","size":5,"soft_fail":false,"target_va":5387333712,"va":5376253643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411c35e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11081466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376253616,"raw_hex":"e8fa16a900","size":5,"soft_fail":false,"target_va":5387335136,"va":5376253665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>
// 记录形式: compiled_procedure; instances=1
// 源码行: 508
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1271,"record_offset":9364,"rva":18626016,"va":5387335136}
// 编译实例 RVAs: 0x11c35e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11656580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387335136,"raw_hex":"e87c224eff","size":5,"soft_fail":false,"target_va":5375678688,"va":5387335263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11657777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387335136,"raw_hex":"e8cf1d4eff","size":5,"soft_fail":false,"target_va":5375678704,"va":5387336476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18613063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387335136,"raw_hex":"e8b9fce3fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5387335250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29014580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387335136,"raw_hex":"e834baba01","size":5,"soft_fail":false,"target_va":5416350832,"va":5387336247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29285508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387335136,"raw_hex":"e884dcbe01","size":5,"soft_fail":false,"target_va":5416620794,"va":5387335281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1271,"record_offset":11008,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1271,"record_offset":9588,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1271,"record_offset":10908,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1271,"record_offset":10312,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1271,"record_offset":11528,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1271,"record_offset":10512,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1271,"record_offset":10344,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1271,"record_offset":10572,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1271,"record_offset":10740,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1271,"record_offset":10228,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":10284,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":22121,"source_line":404,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":9960,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":22149,"source_line":438,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::size_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":10144,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":22145,"source_line":116,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Size::points
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":10628,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":22150,"source_line":417,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":9852,"parent_record_offset":9828,"ancestor_record_offset":9364,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":9876,"parent_record_offset":9852,"ancestor_record_offset":9364,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":11716,"parent_record_offset":11692,"ancestor_record_offset":9364,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":11740,"parent_record_offset":11716,"ancestor_record_offset":9364,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":9900,"parent_record_offset":9876,"ancestor_record_offset":9364,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":11764,"parent_record_offset":11740,"ancestor_record_offset":9364,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":9828,"parent_record_offset":9772,"ancestor_record_offset":9364,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":11692,"parent_record_offset":11600,"ancestor_record_offset":9364,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":11100,"parent_record_offset":11076,"ancestor_record_offset":9364,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":9676,"parent_record_offset":9632,"ancestor_record_offset":9364,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":11628,"parent_record_offset":11600,"ancestor_record_offset":9364,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":11600,"parent_record_offset":11576,"ancestor_record_offset":9364,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":11148,"parent_record_offset":11124,"ancestor_record_offset":9364,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":11124,"parent_record_offset":11100,"ancestor_record_offset":9364,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":9772,"parent_record_offset":9720,"ancestor_record_offset":9364,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":9720,"parent_record_offset":9676,"ancestor_record_offset":9364,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":11576,"parent_record_offset":11552,"ancestor_record_offset":9364,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1271,"record_offset":11076,"parent_record_offset":11036,"ancestor_record_offset":9364,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1271,"record_offset":11036,"parent_record_offset":11008,"ancestor_record_offset":9364,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1271,"record_offset":9632,"parent_record_offset":9588,"ancestor_record_offset":9364,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1271,"record_offset":11660,"parent_record_offset":11628,"ancestor_record_offset":9364,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1271,"record_offset":10192,"parent_record_offset":10168,"ancestor_record_offset":9364,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1271,"record_offset":10252,"parent_record_offset":10228,"ancestor_record_offset":9364,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1271,"record_offset":11552,"parent_record_offset":11528,"ancestor_record_offset":9364,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1271,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":9364,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1271,"record_offset":10540,"parent_record_offset":10512,"ancestor_record_offset":9364,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1271,"record_offset":11188,"parent_record_offset":11036,"ancestor_record_offset":9364,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1271,"record_offset":10368,"parent_record_offset":10344,"ancestor_record_offset":9364,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1271,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":9364,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":10012,"parent_record_offset":9960,"ancestor_record_offset":9364,"inlinee":22121,"source_line":404,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::Rect::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":10168,"parent_record_offset":10144,"ancestor_record_offset":9364,"inlinee":22116,"source_line":81,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>
// 记录形式: compiled_procedure; instances=1
// 源码行: 508
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1271,"record_offset":6736,"rva":18624592,"va":5387333712}
// 编译实例 RVAs: 0x11c3050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11655159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387333712,"raw_hex":"e809284eff","size":5,"soft_fail":false,"target_va":5375678688,"va":5387333842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11656337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387333712,"raw_hex":"e86f234eff","size":5,"soft_fail":false,"target_va":5375678704,"va":5387335036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18611642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387333712,"raw_hex":"e84602e4fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5387333829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29016052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387333712,"raw_hex":"e8f4bfba01","size":5,"soft_fail":false,"target_va":5416350832,"va":5387334775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29286929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387333712,"raw_hex":"e811e2be01","size":5,"soft_fail":false,"target_va":5416620794,"va":5387333860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1271,"record_offset":8380,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1271,"record_offset":6960,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1271,"record_offset":8280,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1271,"record_offset":7684,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1271,"record_offset":8904,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1271,"record_offset":7884,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1271,"record_offset":7716,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1271,"record_offset":7944,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1271,"record_offset":8112,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1271,"record_offset":7600,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":7656,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":22121,"source_line":404,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":7332,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":22149,"source_line":438,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::size_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":7516,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":22145,"source_line":116,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Size::points
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":8000,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":22150,"source_line":417,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":7224,"parent_record_offset":7200,"ancestor_record_offset":6736,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":7248,"parent_record_offset":7224,"ancestor_record_offset":6736,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":9092,"parent_record_offset":9068,"ancestor_record_offset":6736,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":6736,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":6736,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":9140,"parent_record_offset":9116,"ancestor_record_offset":6736,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":7200,"parent_record_offset":7144,"ancestor_record_offset":6736,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":9068,"parent_record_offset":8976,"ancestor_record_offset":6736,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":8476,"parent_record_offset":8452,"ancestor_record_offset":6736,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":7048,"parent_record_offset":7004,"ancestor_record_offset":6736,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":9004,"parent_record_offset":8976,"ancestor_record_offset":6736,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":8976,"parent_record_offset":8952,"ancestor_record_offset":6736,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":8524,"parent_record_offset":8500,"ancestor_record_offset":6736,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":8500,"parent_record_offset":8476,"ancestor_record_offset":6736,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":7144,"parent_record_offset":7092,"ancestor_record_offset":6736,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":7092,"parent_record_offset":7048,"ancestor_record_offset":6736,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":8952,"parent_record_offset":8928,"ancestor_record_offset":6736,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1271,"record_offset":8452,"parent_record_offset":8408,"ancestor_record_offset":6736,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1271,"record_offset":8408,"parent_record_offset":8380,"ancestor_record_offset":6736,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1271,"record_offset":7004,"parent_record_offset":6960,"ancestor_record_offset":6736,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1271,"record_offset":9036,"parent_record_offset":9004,"ancestor_record_offset":6736,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1271,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":6736,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1271,"record_offset":7624,"parent_record_offset":7600,"ancestor_record_offset":6736,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1271,"record_offset":8928,"parent_record_offset":8904,"ancestor_record_offset":6736,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1271,"record_offset":7788,"parent_record_offset":7764,"ancestor_record_offset":6736,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1271,"record_offset":7912,"parent_record_offset":7884,"ancestor_record_offset":6736,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1271,"record_offset":8564,"parent_record_offset":8408,"ancestor_record_offset":6736,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1271,"record_offset":7740,"parent_record_offset":7716,"ancestor_record_offset":6736,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1271,"record_offset":7764,"parent_record_offset":7740,"ancestor_record_offset":6736,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":7384,"parent_record_offset":7332,"ancestor_record_offset":6736,"inlinee":22121,"source_line":404,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::Rect::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1271,"record_offset":7540,"parent_record_offset":7516,"ancestor_record_offset":6736,"inlinee":22117,"source_line":85,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LengthPercentageDeser::Naked::constructor$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 731
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1142B
// 首条位置: {"module":1271,"inlinee":70699}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 735
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5670
// 首条位置: {"module":1271,"inlinee":22128}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_bytes<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 735
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1271,"record_offset":2608,"rva":18623568,"va":5387332688}
// 编译实例 RVAs: 0x11c2c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t29222943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387332688,"raw_hex":"e81fe8bd01","size":5,"soft_fail":false,"target_va":5416555792,"va":5387332844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-11654209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387332688,"raw_hex":"e8bf2b4eff","size":5,"soft_fail":false,"target_va":5375678704,"va":5387332908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::private::string::from_utf8_lossy
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2713916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387332688,"raw_hex":"e8c496d6ff","size":5,"soft_fail":false,"target_va":5384618896,"va":5387332807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1271,"record_offset":2788,"parent_record_offset":2608,"ancestor_record_offset":2608,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":2956,"parent_record_offset":2932,"ancestor_record_offset":2608,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":2608,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":2608,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1271,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":2608,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":2908,"parent_record_offset":2884,"ancestor_record_offset":2608,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1271,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":2608,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1271,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":2608,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 735
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11431
// 首条位置: {"module":2930,"inlinee":70705}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2930,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":712,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2930,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":712,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2930,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":712,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02930.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 735
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11422
// 首条位置: {"module":1271,"inlinee":70690}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$2::visit_enum<serde::private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 735
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1271,"record_offset":1876,"rva":18623184,"va":5387332304}
// 编译实例 RVAs: 0x11c2ad0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t29222278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387332304,"raw_hex":"e886e5bd01","size":5,"soft_fail":false,"target_va":5416554848,"va":5387332565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14238a5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18643659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387332304,"raw_hex":"e8cb7a1c01","size":5,"soft_fail":false,"target_va":5405975984,"va":5387332320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14238a780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18644079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387332304,"raw_hex":"e86f7c1c01","size":5,"soft_fail":false,"target_va":5405976448,"va":5387332364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1271,"record_offset":2376,"parent_record_offset":1876,"ancestor_record_offset":1876,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1271,"record_offset":2148,"parent_record_offset":1876,"ancestor_record_offset":1876,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1271,"record_offset":2412,"parent_record_offset":1876,"ancestor_record_offset":1876,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1271,"record_offset":2120,"parent_record_offset":1876,"ancestor_record_offset":1876,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1271,"record_offset":2184,"parent_record_offset":1876,"ancestor_record_offset":1876,"inlinee":6481,"source_line":2180,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$52::newtype_variant_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1271,"record_offset":2212,"parent_record_offset":2184,"ancestor_record_offset":1876,"inlinee":9610,"source_line":2860,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1271,"record_offset":2312,"parent_record_offset":2276,"ancestor_record_offset":1876,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1271,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":1876,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::impl$26::from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 742
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x566C
// 首条位置: {"module":1271,"inlinee":22124}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::impl$27::from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 767
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11429
// 首条位置: {"module":1271,"inlinee":70697}
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
// 完整结构及原始类型记录: layout.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::Align
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<toolbar::mei::layout::Align>", "constructor": "core::marker::PhantomData", "arguments": ["toolbar::mei::layout::Align"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::Align> >", "constructor": "ron::de::impl$9::newtype_variant_seed", "arguments": ["core::marker::PhantomData<toolbar::mei::layout::Align>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::Axis
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::ComputedLayout
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<toolbar::mei::layout::ComputedLayout,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["toolbar::mei::layout::ComputedLayout", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::LayoutStyle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 18; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>", "constructor": "toolbar::mei::layout::LayoutStyle::place_childen_impl", "arguments": ["toolbar::mei::layout::SwizzleYX"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>", "constructor": "toolbar::mei::layout::LayoutStyle::place_childen_impl", "arguments": ["toolbar::mei::layout::SwizzleXY"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >", "constructor": "toolbar::mei::layout::LayoutStyle::compute_min_size_with_children", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::LayoutTemp
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<toolbar::mei::layout::LayoutTemp,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["toolbar::mei::layout::LayoutTemp", "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["toolbar::mei::layout::LayoutTemp", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$21::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["toolbar::mei::layout::LayoutTemp", "alloc::vec::impl$21::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::layout::LayoutTemp,tuple$<>,toolbar::mei::core::impl$17::perform_layout::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$21::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::layout::LayoutTemp", "tuple$<>", "toolbar::mei::core::impl$17::perform_layout::closure_env$2", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$21::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<country_core::systems::ui::GlobalId>,country_core::systems::ui::GlobalId,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::layout::LayoutTemp,tuple$<>,toolbar::mei::core::impl$17::perform_layout::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$21::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> > > > >", "constructor": "core::iter::adapters::copied::impl$1::fold", "arguments": ["core::slice::iter::Iter<country_core::systems::ui::GlobalId>", "country_core::systems::ui::GlobalId", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::layout::LayoutTemp,tuple$<>,toolbar::mei::core::impl$17::perform_layout::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$21::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::LengthPercentage
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "enum2$<toolbar::mei::layout::LengthPercentage>", "constructor": "enum2$", "arguments": ["toolbar::mei::layout::LengthPercentage"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> >", "constructor": "toolbar::mei::layout::_::impl$0::deserialize", "arguments": ["enum2$<toolbar::mei::layout::LengthPercentage>", "ref_mut$<ron::de::Deserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<enum2$<toolbar::mei::layout::LengthPercentage> >", "constructor": "core::marker::PhantomData", "arguments": ["enum2$<toolbar::mei::layout::LengthPercentage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<enum2$<toolbar::mei::layout::LengthPercentage> > >", "constructor": "ron::de::impl$9::newtype_variant_seed", "arguments": ["core::marker::PhantomData<enum2$<toolbar::mei::layout::LengthPercentage> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> >", "constructor": "toolbar::mei::layout::Rect", "arguments": ["enum2$<toolbar::mei::layout::LengthPercentage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > >", "constructor": "core::marker::PhantomData", "arguments": ["toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > > >", "constructor": "ron::de::impl$9::newtype_variant_seed", "arguments": ["core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> >", "constructor": "serde_core::de::impls::impl$190::deserialize::TupleVisitor", "arguments": ["enum2$<toolbar::mei::layout::LengthPercentage>", "enum2$<toolbar::mei::layout::LengthPercentage>", "enum2$<toolbar::mei::layout::LengthPercentage>", "enum2$<toolbar::mei::layout::LengthPercentage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::private::de::content::impl$41::deserialize_tuple<enum2$<ron::error::Error>,serde_core::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > >", "constructor": "serde::private::de::content::impl$41::deserialize_tuple", "arguments": ["enum2$<ron::error::Error>", "serde_core::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impl$1::fmt<serde_core::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > >", "constructor": "serde_core::de::impl$1::fmt", "arguments": ["serde_core::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::LengthPercentageDeser::Naked
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::Rect
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> >", "constructor": "toolbar::mei::layout::Rect", "arguments": ["enum2$<toolbar::mei::layout::LengthPercentage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > >", "constructor": "core::marker::PhantomData", "arguments": ["toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > > >", "constructor": "ron::de::impl$9::newtype_variant_seed", "arguments": ["core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::Size
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::SwizzleXY
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>", "constructor": "toolbar::mei::layout::LayoutStyle::place_childen_impl", "arguments": ["toolbar::mei::layout::SwizzleXY"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::SwizzleYX
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>", "constructor": "toolbar::mei::layout::LayoutStyle::place_childen_impl", "arguments": ["toolbar::mei::layout::SwizzleYX"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::XAlign
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<toolbar::mei::layout::XAlign>", "constructor": "core::marker::PhantomData", "arguments": ["toolbar::mei::layout::XAlign"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::XAlign> >", "constructor": "ron::de::impl$9::newtype_variant_seed", "arguments": ["core::marker::PhantomData<toolbar::mei::layout::XAlign>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: layout.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: layout.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::PreviewWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::ExpandWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>,tuple$<bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::onboarding::OnboardingDisabled> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>", "bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::PreviewWallSelectionGroup>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::ExpandWallSelectionGroup>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>", "tuple$<bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::onboarding::OnboardingDisabled> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::PreviewWallSelectionGroup>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::ExpandWallSelectionGroup>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::onboarding::OnboardingDisabled>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiPointerCapture>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiPointerCapture>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiPointerCapture>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >", "bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Axis::is_horizontal
// 信息 ID: 43acb836c86480a04822dc18
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::mei_post_update; module=348, inline_record=71496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::ComputedLayout::center
// 信息 ID: 1b886692d7bc9afb4ecb9195
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x311394 -> 0x732080 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::fill_slider @ 0x673471 -> 0x732080 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::core::mei_post_update; module=348, inline_record=68448; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::mei_post_update; module=348, inline_record=75716; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::iter_subtree<tuple$<>,toolbar::mei::core::update_nav::closure_env$2>; module=1235, inline_record=158648; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::align_center
// 信息 ID: 2756f316ae8200f2cd12bc5b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$53::pill_button<ref$<str$> > @ 0x2241a0 -> 0x731e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::text_pill_button @ 0x327ae3 -> 0x731e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x672460 -> 0x731e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::fill_slider @ 0x67338e -> 0x731e80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::align_end
// 信息 ID: a966e2d86f3421c309514258
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x67239b -> 0x732050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >
// 信息 ID: 5ae134398663fed475e73d76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["country_core::systems::ui::GlobalId"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::copied::Copied -> ["core::slice::iter::Iter<country_core::systems::ui::GlobalId>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >", "toolbar::mei::core::impl$17::perform_layout::closure_env$0"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children -> ["core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x729d6e -> 0x11c3b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::expand
// 信息 ID: e14adba95b4587c578b93c47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::impl$5::tod_circle<u64> @ 0x222286 -> 0x731fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::tab_switcher<ref$<str$> > @ 0x2234c1 -> 0x731fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::text_pill_button @ 0x327aee -> 0x731fd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::gap
// 信息 ID: 1e37b5c0e5fe1f9335a19c58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186e41 -> 0x731f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186f19 -> 0x731f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186fe3 -> 0x731f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x1870ad -> 0x731f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x672445 -> 0x731f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x672ad4 -> 0x731f50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::horizontal
// 信息 ID: 52cda7beee6b4fa8118ca047
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::horizontal; module=355, inline_record=496; 不是独立调用边。
// [PDB 内联宿主] toolbar::decoclutter_picker::decoclutter_picker_menu; module=4207, inline_record=6304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::padding
// 信息 ID: cec4d5c3f9e589e18cfe4eaa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::gift_tag<u64> @ 0x5579fd -> 0x732010 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::gift_tag<ref$<str$> > @ 0x557e6d -> 0x732010 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x67240b -> 0x732010 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=356, inline_record=6416; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::place_childen
// 信息 ID: f1f86398b4ad48f6c6b41d20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x72a31c -> 0x731eb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>
// 信息 ID: b597626ae56037cd52a3752b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::layout::LayoutStyle::place_childen_impl -> ["toolbar::mei::layout::SwizzleXY"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::layout::LayoutStyle::place_childen @ 0x731ee1 -> 0x11c35e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>
// 信息 ID: d1b898c910ae859acad03a75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::layout::LayoutStyle::place_childen_impl -> ["toolbar::mei::layout::SwizzleYX"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::layout::LayoutStyle::place_childen @ 0x731ecb -> 0x11c3050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::size
// 信息 ID: ff453bf3294722b86db30674
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$53::pill_button<ref$<str$> > @ 0x2241c5 -> 0x731f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::text_pill_button @ 0x327b08 -> 0x731f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::gift_tag<u64> @ 0x5579c8 -> 0x731f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::gift_tag<ref$<str$> > @ 0x557e38 -> 0x731f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::fill_slider @ 0x6733c8 -> 0x731f90 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::decoclutter_picker::decoclutter_picker_menu; module=4207, inline_record=6336; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::space_between
// 信息 ID: 2091a75049c5d9bcfc1b94c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_impl<ref$<str$> > @ 0x224480 -> 0x731ef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::vertical
// 信息 ID: 2ec165f8781312af855e0774
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::vertical; module=355, inline_record=2016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::xalign_center
// 信息 ID: 56f66450c2794e0dcdac037a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$53::pill_button<ref$<str$> > @ 0x2241ab -> 0x731f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x6723b6 -> 0x731f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x672478 -> 0x731f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::fill_slider @ 0x67339e -> 0x731f20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LengthPercentage::points
// 信息 ID: a3b25e8ca3ce2e7cd4aca00a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 22 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1271, inline_record=7384; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1271, inline_record=7416; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1271, inline_record=7448; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1271, inline_record=7480; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1271, inline_record=7656; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1271, inline_record=8024; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1271, inline_record=8080; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1271, inline_record=8628; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LengthPercentage::points_or_zero
// 信息 ID: 902eab898cec267a57d88240
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15120; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15376; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15408; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15472; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15504; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15608; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15640; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15704; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LengthPercentageDeser::Naked::constructor$0
// 信息 ID: 484cb583d4da7fb15acf8f19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >; module=1271, inline_record=17324; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >; module=1271, inline_record=17952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Rect::height
// 信息 ID: e458e1f4cc7bb37779ca7490
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1271, inline_record=7540; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Rect::points
// 信息 ID: 63ce5576039afd758b5f4fce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1271, inline_record=7332; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1271, inline_record=9960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Rect::points_or_zero
// 信息 ID: 7f34a93aa51fa5e598a0582f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Rect::size_vec2
// 信息 ID: 23719861a44757eccb1b74f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1271, inline_record=7516; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1271, inline_record=10144; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Rect::width
// 信息 ID: 3a754a8263603dba8b4f4b50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1271, inline_record=10168; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Size::new
// 信息 ID: 286eea36652483a3323683cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::text_pill_button; module=90, inline_record=1508; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::gift_tag<u64>; module=196, inline_record=1044; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::gift_tag<u64>; module=196, inline_record=1072; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::gift_tag<ref$<str$> >; module=196, inline_record=1740; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::gift_tag<ref$<str$> >; module=196, inline_record=1768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Size::points
// 信息 ID: d9f096b90aea065778cde75f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1271, inline_record=8000; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1271, inline_record=8056; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1271, inline_record=8600; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1271, inline_record=8696; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1271, inline_record=10628; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1271, inline_record=10684; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1271, inline_record=11224; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1271, inline_record=11320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Size::points_or_zero
// 信息 ID: 812be456837bac9e0c84c639
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15348; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15444; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15580; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1271, inline_record=15676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize
// 信息 ID: f4cf65e6b08cc65ffbd5c793
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> >; module=1271, inline_record=16520; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >; module=1271, inline_record=17132; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >; module=1271, inline_record=17384; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >; module=1271, inline_record=17744; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >; module=1271, inline_record=18012; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::Align> >; module=2887, inline_record=27168; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::XAlign> >; module=2887, inline_record=27824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: a96f2b003cae8101d5982144
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<toolbar::mei::layout::_::impl$0::deserialize::__FieldVisitor>; module=1271, inline_record=3528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_bytes<enum2$<ron::error::Error> >
// 信息 ID: cedad487333c8e181571c4ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_bytes -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: fd944503bb87286976be93a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2930, inline_record=712; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2930, inline_record=1220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 9d737457c01d5297ab554956
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> > @ 0x2330436 -> 0x11c28d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> > @ 0x23308b6 -> 0x11c29f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: b43def6da4aead2797e32c05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2930, inline_record=660; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2930, inline_record=996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 574243461839ef83514e5559
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2887, inline_record=13872; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2887, inline_record=15580; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2930, inline_record=544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: b0bc99d1e647137fa8bb8925
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<toolbar::mei::layout::_::impl$0::deserialize::__Visitor>; module=2887, inline_record=77844; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<toolbar::mei::layout::_::impl$0::deserialize::__Visitor>; module=2887, inline_record=82692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$2::visit_enum<serde::private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> > >
// 信息 ID: b7fe62074bb15055abeb6354
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] serde::private::de::content::EnumRefDeserializer -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize::impl$2::visit_enum -> ["serde::private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_enum<enum2$<ron::error::Error>,toolbar::mei::layout::_::impl$0::deserialize::__Visitor> @ 0x238a8b3 -> 0x11c2ad0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> >
// 信息 ID: 28df4f0d4688e0185dd0d45d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["toolbar::mei::layout::LengthPercentage"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ron::de::Deserializer"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize -> ["enum2$<toolbar::mei::layout::LengthPercentage>", "ref_mut$<ron::de::Deserializer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > > > @ 0x2333585 -> 0x11c4010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >
// 信息 ID: bb1b79bce9fa099e9e38b113
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::Deserializer"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize -> ["ref_mut$<ron::de::Deserializer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$9::tuple_variant<toolbar::mei::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x233113e -> 0x11c4540 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$9::tuple_variant<toolbar::mei::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x23311ec -> 0x11c4540 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<enum2$<toolbar::mei::layout::LengthPercentage> > > @ 0x2332b2b -> 0x11c4540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize<serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >
// 信息 ID: 61b02f40e98bccc07bb43e4f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] serde::private::de::content::ContentRefDeserializer -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize -> ["serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> > @ 0x11c408a -> 0x11c4330 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_tuple<enum2$<ron::error::Error>,serde_core::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > > @ 0x238a95d -> 0x11c4330 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_tuple<enum2$<ron::error::Error>,serde_core::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > > @ 0x238a98c -> 0x11c4330 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_tuple<enum2$<ron::error::Error>,serde_core::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > > @ 0x238a9bc -> 0x11c4330 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_tuple<enum2$<ron::error::Error>,serde_core::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > > @ 0x238a9f0 -> 0x11c4330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::horizontal
// 信息 ID: c3c9484989c61010c53ff350
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 25 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186e1d -> 0x731e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186f00 -> 0x731e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186fca -> 0x731e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x18709c -> 0x731e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_impl<ref$<str$> > @ 0x224470 -> 0x731e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x224653 -> 0x731e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x224d1f -> 0x731e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode> > @ 0x2252af -> 0x731e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x22583f -> 0x731e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingPreset> > @ 0x225e2f -> 0x731e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x2263c1 -> 0x731e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x22689d -> 0x731e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x226daf -> 0x731e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > > @ 0x22740f -> 0x731e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x2279ff -> 0x731e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<country_core::render::render_config::ImageQualityPreset> > @ 0x227f8f -> 0x731e50 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::decoclutter_picker::decoclutter_picker_menu; module=4207, inline_record=6280; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::impl$11::from_points
// 信息 ID: 82f116d5c29cc1e1be8aba4f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::impl$13::from_points
// 信息 ID: ad6f66754900a6f9c020854e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::impl$16::default
// 信息 ID: ff10026d4029f315a210d927
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::impl$5::tod_circle<u64> @ 0x222273 -> 0x731e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::tab_switcher<ref$<str$> > @ 0x2234ae -> 0x731e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$53::pill_button<ref$<str$> > @ 0x224191 -> 0x731e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::fill_slider @ 0x67337b -> 0x731e20 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=356, inline_record=6324; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::impl$26::from
// 信息 ID: f8dfddc20ed2ff1ce4afb265
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >; module=1271, inline_record=17536; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >; module=1271, inline_record=18172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::impl$27::from
// 信息 ID: e767b1f1331d67e2321c6055
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> >; module=1271, inline_record=16876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::vertical
// 信息 ID: db41d8d84b33f7c640a6d640
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x67241f -> 0x7320a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
