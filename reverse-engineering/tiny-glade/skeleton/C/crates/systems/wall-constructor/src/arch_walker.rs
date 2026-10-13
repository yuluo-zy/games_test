// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/arch_walker.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 093ebc0b0b18bc34bd034563571fce4ef5200f340474b22820e80e935ed15dc4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: arch_walker.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 62, 394, 397, 420, 1462, 1495, 1511
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::ArchWalker::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=9
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65F6
// 首条位置: {"module":394,"inlinee":26102}
// 编译实例 RVAs: 0x7f7d80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1462,"record_offset":9176,"rva":21038544,"va":5389747664}
// 编译实例 RVAs: 0x14105d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140de61f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6465626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389747664,"raw_hex":"e8a6579dff","size":5,"soft_fail":false,"target_va":5383283184,"va":5389748805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de63a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6465173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389747664,"raw_hex":"e86b599dff","size":5,"soft_fail":false,"target_va":5383283616,"va":5389748784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6422826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389747664,"raw_hex":"e8d6fe9dff","size":5,"soft_fail":false,"target_va":5383325056,"va":5389747877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27309587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389747664,"raw_hex":"e813b6a001","size":5,"soft_fail":false,"target_va":5417057792,"va":5389748200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26872685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389747664,"raw_hex":"e86d0b9a01","size":5,"soft_fail":false,"target_va":5416622208,"va":5389749518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26873953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389747664,"raw_hex":"e861109a01","size":5,"soft_fail":false,"target_va":5416621762,"va":5389747804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26872741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389747664,"raw_hex":"e8a50b9a01","size":5,"soft_fail":false,"target_va":5416622240,"va":5389749494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_arches::construct_arches::closure$3
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389747664,"raw_hex":"e893090000","size":5,"soft_fail":false,"target_va":5389751440,"va":5389748984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":10272,"parent_record_offset":9176,"ancestor_record_offset":9176,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":1462,"record_offset":9592,"parent_record_offset":9176,"ancestor_record_offset":9176,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":9452,"parent_record_offset":9176,"ancestor_record_offset":9176,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1462,"record_offset":10820,"parent_record_offset":9176,"ancestor_record_offset":9176,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":10088,"parent_record_offset":9176,"ancestor_record_offset":9176,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":10000,"parent_record_offset":9176,"ancestor_record_offset":9176,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1462,"record_offset":10216,"parent_record_offset":9176,"ancestor_record_offset":9176,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1462,"record_offset":10904,"parent_record_offset":9176,"ancestor_record_offset":9176,"inlinee":9790,"source_line":122,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::curve_to_arch_ws
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1462,"record_offset":9424,"parent_record_offset":9176,"ancestor_record_offset":9176,"inlinee":9771,"source_line":163,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":9548,"parent_record_offset":9524,"ancestor_record_offset":9176,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":11300,"parent_record_offset":11276,"ancestor_record_offset":9176,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":11324,"parent_record_offset":11300,"ancestor_record_offset":9176,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":9524,"parent_record_offset":9500,"ancestor_record_offset":9176,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":11160,"parent_record_offset":10904,"ancestor_record_offset":9176,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":10240,"parent_record_offset":10216,"ancestor_record_offset":9176,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":10176,"parent_record_offset":10152,"ancestor_record_offset":9176,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1462,"record_offset":9628,"parent_record_offset":9592,"ancestor_record_offset":9176,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1462,"record_offset":10536,"parent_record_offset":10468,"ancestor_record_offset":9176,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1462,"record_offset":10692,"parent_record_offset":10668,"ancestor_record_offset":9176,"inlinee":5091,"source_line":115,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":9476,"parent_record_offset":9452,"ancestor_record_offset":9176,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::super_nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":9848,"parent_record_offset":9744,"ancestor_record_offset":9176,"inlinee":9784,"source_line":26,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":9660,"parent_record_offset":9592,"ancestor_record_offset":9176,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":9720,"parent_record_offset":9592,"ancestor_record_offset":9176,"inlinee":9782,"source_line":94,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":9500,"parent_record_offset":9476,"ancestor_record_offset":9176,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":9684,"parent_record_offset":9660,"ancestor_record_offset":9176,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":9744,"parent_record_offset":9720,"ancestor_record_offset":9176,"inlinee":9783,"source_line":327,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":10644,"parent_record_offset":10620,"ancestor_record_offset":9176,"inlinee":6891,"source_line":306,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by::impl$1::spec_advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":10668,"parent_record_offset":10644,"ancestor_record_offset":9176,"inlinee":6889,"source_line":325,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":10620,"parent_record_offset":10468,"ancestor_record_offset":9176,"inlinee":6943,"source_line":384,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":9176,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1462,"record_offset":10844,"parent_record_offset":10820,"ancestor_record_offset":9176,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1462,"record_offset":11424,"parent_record_offset":11392,"ancestor_record_offset":9176,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1462,"record_offset":10028,"parent_record_offset":10000,"ancestor_record_offset":9176,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1462,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":9176,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1462,"record_offset":11252,"parent_record_offset":11216,"ancestor_record_offset":9176,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":10116,"parent_record_offset":10088,"ancestor_record_offset":9176,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":11040,"parent_record_offset":10904,"ancestor_record_offset":9176,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":11368,"parent_record_offset":10904,"ancestor_record_offset":9176,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":11392,"parent_record_offset":11368,"ancestor_record_offset":9176,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":11540,"parent_record_offset":11368,"ancestor_record_offset":9176,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":10984,"parent_record_offset":10904,"ancestor_record_offset":9176,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1462,"record_offset":10152,"parent_record_offset":10116,"ancestor_record_offset":9176,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1462,"record_offset":11188,"parent_record_offset":10904,"ancestor_record_offset":9176,"inlinee":9792,"source_line":25,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1462,"record_offset":11216,"parent_record_offset":10904,"ancestor_record_offset":9176,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1462,"record_offset":280,"rva":21033792,"va":5389742912}
// 编译实例 RVAs: 0x140f340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=2
//     target_aliases=20 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11617712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389742912,"raw_hex":"e850ba4eff","size":5,"soft_fail":false,"target_va":5378126944,"va":5389744651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c6a170
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8018096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389742912,"raw_hex":"e850a785ff","size":5,"soft_fail":false,"target_va":5381726576,"va":5389744667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de61f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6460975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389742912,"raw_hex":"e8d1699dff","size":5,"soft_fail":false,"target_va":5383283184,"va":5389744154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de63a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6460522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389742912,"raw_hex":"e8966b9dff","size":5,"soft_fail":false,"target_va":5383283616,"va":5389744133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6418083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389742912,"raw_hex":"e85d119eff","size":5,"soft_fail":false,"target_va":5383325056,"va":5389743134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141403450
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-50766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389742912,"raw_hex":"e8b239ffff","size":5,"soft_fail":false,"target_va":5389694032,"va":5389744793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27314324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389742912,"raw_hex":"e894c8a001","size":5,"soft_fail":false,"target_va":5417057792,"va":5389743463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26876457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389742912,"raw_hex":"e8291a9a01","size":5,"soft_fail":false,"target_va":5416622208,"va":5389745746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26878705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389742912,"raw_hex":"e8f1229a01","size":5,"soft_fail":false,"target_va":5416621762,"va":5389743052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26876513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389742912,"raw_hex":"e8611a9a01","size":5,"soft_fail":false,"target_va":5416622240,"va":5389745722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-6336280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389742912,"raw_hex":"e8e8509fff","size":5,"soft_fail":false,"target_va":5383408256,"va":5389744531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":1484,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":1462,"record_offset":804,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":664,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1462,"record_offset":2032,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":1300,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":1212,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1462,"record_offset":1428,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1462,"record_offset":2116,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9790,"source_line":122,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::curve_to_arch_ws
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1462,"record_offset":636,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9771,"source_line":163,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1462,"record_offset":4540,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79469,"source_line":567,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":280,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":280,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":2520,"parent_record_offset":2496,"ancestor_record_offset":280,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":280,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1462,"record_offset":2980,"parent_record_offset":2564,"ancestor_record_offset":280,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1462,"record_offset":3600,"parent_record_offset":3544,"ancestor_record_offset":280,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1462,"record_offset":3096,"parent_record_offset":3064,"ancestor_record_offset":280,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1462,"record_offset":3180,"parent_record_offset":3064,"ancestor_record_offset":280,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1462,"record_offset":3152,"parent_record_offset":3064,"ancestor_record_offset":280,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":2360,"parent_record_offset":2116,"ancestor_record_offset":280,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":2916,"parent_record_offset":2888,"ancestor_record_offset":280,"inlinee":9829,"source_line":907,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":280,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":280,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1462,"record_offset":840,"parent_record_offset":804,"ancestor_record_offset":280,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1462,"record_offset":1748,"parent_record_offset":1680,"ancestor_record_offset":280,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1462,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":280,"inlinee":5091,"source_line":115,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":280,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::super_nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":1060,"parent_record_offset":956,"ancestor_record_offset":280,"inlinee":9784,"source_line":26,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":872,"parent_record_offset":804,"ancestor_record_offset":280,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":932,"parent_record_offset":804,"ancestor_record_offset":280,"inlinee":9782,"source_line":94,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":280,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":280,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":280,"inlinee":9783,"source_line":327,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":280,"inlinee":6891,"source_line":306,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by::impl$1::spec_advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":280,"inlinee":6889,"source_line":325,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":1832,"parent_record_offset":1680,"ancestor_record_offset":280,"inlinee":6943,"source_line":384,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":280,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1462,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1462,"record_offset":2944,"parent_record_offset":2888,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1462,"record_offset":1240,"parent_record_offset":1212,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1462,"record_offset":2472,"parent_record_offset":2448,"ancestor_record_offset":280,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1462,"record_offset":2448,"parent_record_offset":2416,"ancestor_record_offset":280,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":1462,"record_offset":3312,"parent_record_offset":2564,"ancestor_record_offset":280,"inlinee":79470,"source_line":149,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::impl$8::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":1462,"record_offset":3424,"parent_record_offset":2564,"ancestor_record_offset":280,"inlinee":79474,"source_line":565,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_diagonal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1462,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":280,"inlinee":79471,"source_line":160,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1462,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":280,"inlinee":79479,"source_line":707,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1462,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":280,"inlinee":79472,"source_line":78,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$16::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1462,"record_offset":3460,"parent_record_offset":3424,"ancestor_record_offset":280,"inlinee":79476,"source_line":918,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$22::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1462,"record_offset":3492,"parent_record_offset":3460,"ancestor_record_offset":280,"inlinee":79478,"source_line":968,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1462,"record_offset":3064,"parent_record_offset":2564,"ancestor_record_offset":280,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1462,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":280,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1462,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":280,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$19::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1462,"record_offset":3544,"parent_record_offset":3516,"ancestor_record_offset":280,"inlinee":79481,"source_line":1453,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1462,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":280,"inlinee":27159,"source_line":1577,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":1328,"parent_record_offset":1300,"ancestor_record_offset":280,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":2240,"parent_record_offset":2116,"ancestor_record_offset":280,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":280,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":2864,"parent_record_offset":2564,"ancestor_record_offset":280,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":2184,"parent_record_offset":2116,"ancestor_record_offset":280,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1462,"record_offset":3780,"parent_record_offset":3752,"ancestor_record_offset":280,"inlinee":79483,"source_line":180,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1462,"record_offset":1364,"parent_record_offset":1328,"ancestor_record_offset":280,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::construct_lintels::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/mod.rs
//     evidence={"module":1462,"record_offset":4472,"parent_record_offset":2564,"ancestor_record_offset":280,"inlinee":79484,"source_line":92,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1462,"record_offset":2564,"parent_record_offset":2116,"ancestor_record_offset":280,"inlinee":79469,"source_line":567,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1462,"record_offset":2388,"parent_record_offset":2116,"ancestor_record_offset":280,"inlinee":9792,"source_line":25,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1462,"record_offset":2416,"parent_record_offset":2116,"ancestor_record_offset":280,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$8>
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1462,"record_offset":11700,"rva":21040432,"va":5389749552}
// 编译实例 RVAs: 0x1410d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140de61f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6467514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389749552,"raw_hex":"e846509dff","size":5,"soft_fail":false,"target_va":5383283184,"va":5389750693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de63a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6467061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389749552,"raw_hex":"e80b529dff","size":5,"soft_fail":false,"target_va":5383283616,"va":5389750672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6424714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389749552,"raw_hex":"e876f79dff","size":5,"soft_fail":false,"target_va":5383325056,"va":5389749765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27307699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389749552,"raw_hex":"e8b3aea001","size":5,"soft_fail":false,"target_va":5417057792,"va":5389750088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26870797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389749552,"raw_hex":"e80d049a01","size":5,"soft_fail":false,"target_va":5416622208,"va":5389751406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26872065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389749552,"raw_hex":"e801099a01","size":5,"soft_fail":false,"target_va":5416621762,"va":5389749692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26870853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389749552,"raw_hex":"e845049a01","size":5,"soft_fail":false,"target_va":5416622240,"va":5389751382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure$8
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389749552,"raw_hex":"e813100000","size":5,"soft_fail":false,"target_va":5389754992,"va":5389750872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":12836,"parent_record_offset":11700,"ancestor_record_offset":11700,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":1462,"record_offset":12156,"parent_record_offset":11700,"ancestor_record_offset":11700,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":12016,"parent_record_offset":11700,"ancestor_record_offset":11700,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1462,"record_offset":13384,"parent_record_offset":11700,"ancestor_record_offset":11700,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":12652,"parent_record_offset":11700,"ancestor_record_offset":11700,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":12564,"parent_record_offset":11700,"ancestor_record_offset":11700,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1462,"record_offset":12780,"parent_record_offset":11700,"ancestor_record_offset":11700,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1462,"record_offset":13468,"parent_record_offset":11700,"ancestor_record_offset":11700,"inlinee":9790,"source_line":122,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::curve_to_arch_ws
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1462,"record_offset":11988,"parent_record_offset":11700,"ancestor_record_offset":11700,"inlinee":9771,"source_line":163,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":12112,"parent_record_offset":12088,"ancestor_record_offset":11700,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":11700,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":13888,"parent_record_offset":13864,"ancestor_record_offset":11700,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":12088,"parent_record_offset":12064,"ancestor_record_offset":11700,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":13724,"parent_record_offset":13468,"ancestor_record_offset":11700,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":12804,"parent_record_offset":12780,"ancestor_record_offset":11700,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":12740,"parent_record_offset":12716,"ancestor_record_offset":11700,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1462,"record_offset":12192,"parent_record_offset":12156,"ancestor_record_offset":11700,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1462,"record_offset":13100,"parent_record_offset":13032,"ancestor_record_offset":11700,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1462,"record_offset":13256,"parent_record_offset":13232,"ancestor_record_offset":11700,"inlinee":5091,"source_line":115,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":12040,"parent_record_offset":12016,"ancestor_record_offset":11700,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::super_nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":12412,"parent_record_offset":12308,"ancestor_record_offset":11700,"inlinee":9784,"source_line":26,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":12224,"parent_record_offset":12156,"ancestor_record_offset":11700,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":12284,"parent_record_offset":12156,"ancestor_record_offset":11700,"inlinee":9782,"source_line":94,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":12064,"parent_record_offset":12040,"ancestor_record_offset":11700,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":12248,"parent_record_offset":12224,"ancestor_record_offset":11700,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":12308,"parent_record_offset":12284,"ancestor_record_offset":11700,"inlinee":9783,"source_line":327,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":13208,"parent_record_offset":13184,"ancestor_record_offset":11700,"inlinee":6891,"source_line":306,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by::impl$1::spec_advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":13232,"parent_record_offset":13208,"ancestor_record_offset":11700,"inlinee":6889,"source_line":325,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":13184,"parent_record_offset":13032,"ancestor_record_offset":11700,"inlinee":6943,"source_line":384,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":13280,"parent_record_offset":13256,"ancestor_record_offset":11700,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1462,"record_offset":13408,"parent_record_offset":13384,"ancestor_record_offset":11700,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1462,"record_offset":13988,"parent_record_offset":13956,"ancestor_record_offset":11700,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1462,"record_offset":12592,"parent_record_offset":12564,"ancestor_record_offset":11700,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1462,"record_offset":13840,"parent_record_offset":13816,"ancestor_record_offset":11700,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1462,"record_offset":13816,"parent_record_offset":13780,"ancestor_record_offset":11700,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":12680,"parent_record_offset":12652,"ancestor_record_offset":11700,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":13604,"parent_record_offset":13468,"ancestor_record_offset":11700,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":13932,"parent_record_offset":13468,"ancestor_record_offset":11700,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":13956,"parent_record_offset":13932,"ancestor_record_offset":11700,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":14104,"parent_record_offset":13932,"ancestor_record_offset":11700,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":13548,"parent_record_offset":13468,"ancestor_record_offset":11700,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1462,"record_offset":12716,"parent_record_offset":12680,"ancestor_record_offset":11700,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1462,"record_offset":13752,"parent_record_offset":13468,"ancestor_record_offset":11700,"inlinee":9792,"source_line":25,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1462,"record_offset":13780,"parent_record_offset":13468,"ancestor_record_offset":11700,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<true> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":62,"record_offset":248,"rva":2030240,"va":5370739360}
// 编译实例 RVAs: 0x1efaa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140de61f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12542678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370739360,"raw_hex":"e8d662bf00","size":5,"soft_fail":false,"target_va":5383283184,"va":5370740501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de63a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12543131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370739360,"raw_hex":"e89b64bf00","size":5,"soft_fail":false,"target_va":5383283616,"va":5370740480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12585478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370739360,"raw_hex":"e8060ac000","size":5,"soft_fail":false,"target_va":5383325056,"va":5370739573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46317891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370739360,"raw_hex":"e843c1c202","size":5,"soft_fail":false,"target_va":5417057792,"va":5370739896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t45880989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370739360,"raw_hex":"e89d16bc02","size":5,"soft_fail":false,"target_va":5416622208,"va":5370741214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t45882257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370739360,"raw_hex":"e8911bbc02","size":5,"soft_fail":false,"target_va":5416621762,"va":5370739500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t45881045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370739360,"raw_hex":"e8d516bc02","size":5,"soft_fail":false,"target_va":5416622240,"va":5370741190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$6<true>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370739360,"raw_hex":"e833020000","size":5,"soft_fail":false,"target_va":5370741248,"va":5370740680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":62,"record_offset":1388,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":62,"record_offset":708,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":62,"record_offset":568,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":62,"record_offset":1936,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":62,"record_offset":1204,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":62,"record_offset":1116,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":62,"record_offset":1332,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":62,"record_offset":2020,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9790,"source_line":122,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::curve_to_arch_ws
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":62,"record_offset":540,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9771,"source_line":163,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":62,"record_offset":664,"parent_record_offset":640,"ancestor_record_offset":248,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":62,"record_offset":2428,"parent_record_offset":2396,"ancestor_record_offset":248,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":62,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":248,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":62,"record_offset":640,"parent_record_offset":616,"ancestor_record_offset":248,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":62,"record_offset":2276,"parent_record_offset":2020,"ancestor_record_offset":248,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":62,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":248,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":62,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":248,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":62,"record_offset":744,"parent_record_offset":708,"ancestor_record_offset":248,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":62,"record_offset":1652,"parent_record_offset":1584,"ancestor_record_offset":248,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":62,"record_offset":1808,"parent_record_offset":1784,"ancestor_record_offset":248,"inlinee":5091,"source_line":115,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":62,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":248,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::super_nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":62,"record_offset":964,"parent_record_offset":860,"ancestor_record_offset":248,"inlinee":9784,"source_line":26,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":62,"record_offset":776,"parent_record_offset":708,"ancestor_record_offset":248,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":62,"record_offset":836,"parent_record_offset":708,"ancestor_record_offset":248,"inlinee":9782,"source_line":94,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":62,"record_offset":616,"parent_record_offset":592,"ancestor_record_offset":248,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":62,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":248,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":62,"record_offset":860,"parent_record_offset":836,"ancestor_record_offset":248,"inlinee":9783,"source_line":327,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":62,"record_offset":1760,"parent_record_offset":1736,"ancestor_record_offset":248,"inlinee":6891,"source_line":306,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by::impl$1::spec_advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":62,"record_offset":1784,"parent_record_offset":1760,"ancestor_record_offset":248,"inlinee":6889,"source_line":325,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":62,"record_offset":1736,"parent_record_offset":1584,"ancestor_record_offset":248,"inlinee":6943,"source_line":384,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":62,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":248,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":62,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":248,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":62,"record_offset":2552,"parent_record_offset":2520,"ancestor_record_offset":248,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":62,"record_offset":1144,"parent_record_offset":1116,"ancestor_record_offset":248,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":62,"record_offset":2396,"parent_record_offset":2368,"ancestor_record_offset":248,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":62,"record_offset":2368,"parent_record_offset":2332,"ancestor_record_offset":248,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":62,"record_offset":1232,"parent_record_offset":1204,"ancestor_record_offset":248,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":62,"record_offset":2156,"parent_record_offset":2020,"ancestor_record_offset":248,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":62,"record_offset":2496,"parent_record_offset":2020,"ancestor_record_offset":248,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":62,"record_offset":2520,"parent_record_offset":2496,"ancestor_record_offset":248,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":62,"record_offset":2668,"parent_record_offset":2496,"ancestor_record_offset":248,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":62,"record_offset":2100,"parent_record_offset":2020,"ancestor_record_offset":248,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":62,"record_offset":1268,"parent_record_offset":1232,"ancestor_record_offset":248,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":62,"record_offset":2304,"parent_record_offset":2020,"ancestor_record_offset":248,"inlinee":9792,"source_line":25,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":62,"record_offset":2332,"parent_record_offset":2020,"ancestor_record_offset":248,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1462,"record_offset":6524,"rva":21036656,"va":5389745776}
// 编译实例 RVAs: 0x140fe70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140de61f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6463738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389745776,"raw_hex":"e8065f9dff","size":5,"soft_fail":false,"target_va":5383283184,"va":5389746917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de63a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6463285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389745776,"raw_hex":"e8cb609dff","size":5,"soft_fail":false,"target_va":5383283616,"va":5389746896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6420938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389745776,"raw_hex":"e836069eff","size":5,"soft_fail":false,"target_va":5383325056,"va":5389745989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27311475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389745776,"raw_hex":"e873bda001","size":5,"soft_fail":false,"target_va":5417057792,"va":5389746312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26874573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389745776,"raw_hex":"e8cd129a01","size":5,"soft_fail":false,"target_va":5416622208,"va":5389747630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26875841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389745776,"raw_hex":"e8c1179a01","size":5,"soft_fail":false,"target_va":5416621762,"va":5389745916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26874629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389745776,"raw_hex":"e805139a01","size":5,"soft_fail":false,"target_va":5416622240,"va":5389747606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389745776,"raw_hex":"e853140000","size":5,"soft_fail":false,"target_va":5389752304,"va":5389747096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":7748,"parent_record_offset":6524,"ancestor_record_offset":6524,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":1462,"record_offset":7068,"parent_record_offset":6524,"ancestor_record_offset":6524,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":6928,"parent_record_offset":6524,"ancestor_record_offset":6524,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1462,"record_offset":8296,"parent_record_offset":6524,"ancestor_record_offset":6524,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":7564,"parent_record_offset":6524,"ancestor_record_offset":6524,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":7476,"parent_record_offset":6524,"ancestor_record_offset":6524,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1462,"record_offset":7692,"parent_record_offset":6524,"ancestor_record_offset":6524,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1462,"record_offset":8380,"parent_record_offset":6524,"ancestor_record_offset":6524,"inlinee":9790,"source_line":122,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::curve_to_arch_ws
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1462,"record_offset":6900,"parent_record_offset":6524,"ancestor_record_offset":6524,"inlinee":9771,"source_line":163,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":7024,"parent_record_offset":7000,"ancestor_record_offset":6524,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":8776,"parent_record_offset":8752,"ancestor_record_offset":6524,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":8800,"parent_record_offset":8776,"ancestor_record_offset":6524,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1462,"record_offset":7000,"parent_record_offset":6976,"ancestor_record_offset":6524,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":8636,"parent_record_offset":8380,"ancestor_record_offset":6524,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":7716,"parent_record_offset":7692,"ancestor_record_offset":6524,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":7652,"parent_record_offset":7628,"ancestor_record_offset":6524,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1462,"record_offset":7104,"parent_record_offset":7068,"ancestor_record_offset":6524,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1462,"record_offset":8012,"parent_record_offset":7944,"ancestor_record_offset":6524,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1462,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":6524,"inlinee":5091,"source_line":115,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":6952,"parent_record_offset":6928,"ancestor_record_offset":6524,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::super_nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":7324,"parent_record_offset":7220,"ancestor_record_offset":6524,"inlinee":9784,"source_line":26,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":7136,"parent_record_offset":7068,"ancestor_record_offset":6524,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":7196,"parent_record_offset":7068,"ancestor_record_offset":6524,"inlinee":9782,"source_line":94,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":6976,"parent_record_offset":6952,"ancestor_record_offset":6524,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":6524,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1462,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":6524,"inlinee":9783,"source_line":327,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":8120,"parent_record_offset":8096,"ancestor_record_offset":6524,"inlinee":6891,"source_line":306,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by::impl$1::spec_advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":8144,"parent_record_offset":8120,"ancestor_record_offset":6524,"inlinee":6889,"source_line":325,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":8096,"parent_record_offset":7944,"ancestor_record_offset":6524,"inlinee":6943,"source_line":384,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1462,"record_offset":8192,"parent_record_offset":8168,"ancestor_record_offset":6524,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1462,"record_offset":8320,"parent_record_offset":8296,"ancestor_record_offset":6524,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1462,"record_offset":8900,"parent_record_offset":8868,"ancestor_record_offset":6524,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1462,"record_offset":7504,"parent_record_offset":7476,"ancestor_record_offset":6524,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1462,"record_offset":8752,"parent_record_offset":8728,"ancestor_record_offset":6524,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1462,"record_offset":8728,"parent_record_offset":8692,"ancestor_record_offset":6524,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":7592,"parent_record_offset":7564,"ancestor_record_offset":6524,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":8516,"parent_record_offset":8380,"ancestor_record_offset":6524,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":8844,"parent_record_offset":8380,"ancestor_record_offset":6524,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":8868,"parent_record_offset":8844,"ancestor_record_offset":6524,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":9016,"parent_record_offset":8844,"ancestor_record_offset":6524,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":8460,"parent_record_offset":8380,"ancestor_record_offset":6524,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1462,"record_offset":7628,"parent_record_offset":7592,"ancestor_record_offset":6524,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1462,"record_offset":8664,"parent_record_offset":8380,"ancestor_record_offset":6524,"inlinee":9792,"source_line":25,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1462,"record_offset":8692,"parent_record_offset":8380,"ancestor_record_offset":6524,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x263E
// 首条位置: {"module":62,"inlinee":9790}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":62,"record_offset":2276,"parent_record_offset":2020,"ancestor_record_offset":2020,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":62,"record_offset":2156,"parent_record_offset":2020,"ancestor_record_offset":2020,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":62,"record_offset":2496,"parent_record_offset":2020,"ancestor_record_offset":2020,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":62,"record_offset":2100,"parent_record_offset":2020,"ancestor_record_offset":2020,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1462,"record_offset":2564,"parent_record_offset":2116,"ancestor_record_offset":2116,"inlinee":79469,"source_line":567,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":62,"record_offset":2304,"parent_record_offset":2020,"ancestor_record_offset":2020,"inlinee":9792,"source_line":25,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":62,"record_offset":2332,"parent_record_offset":2020,"ancestor_record_offset":2020,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":62,"record_offset":2428,"parent_record_offset":2396,"ancestor_record_offset":2020,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":62,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":2020,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1462,"record_offset":2980,"parent_record_offset":2564,"ancestor_record_offset":2116,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1462,"record_offset":3600,"parent_record_offset":3544,"ancestor_record_offset":2116,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1462,"record_offset":3096,"parent_record_offset":3064,"ancestor_record_offset":2116,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1462,"record_offset":3180,"parent_record_offset":3064,"ancestor_record_offset":2116,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1462,"record_offset":3152,"parent_record_offset":3064,"ancestor_record_offset":2116,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":2916,"parent_record_offset":2888,"ancestor_record_offset":2116,"inlinee":9829,"source_line":907,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":62,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":2020,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":62,"record_offset":2700,"parent_record_offset":2668,"ancestor_record_offset":2020,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":62,"record_offset":2552,"parent_record_offset":2520,"ancestor_record_offset":2020,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":62,"record_offset":2124,"parent_record_offset":2100,"ancestor_record_offset":2020,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":62,"record_offset":2396,"parent_record_offset":2368,"ancestor_record_offset":2020,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":62,"record_offset":2368,"parent_record_offset":2332,"ancestor_record_offset":2020,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":1462,"record_offset":3312,"parent_record_offset":2564,"ancestor_record_offset":2116,"inlinee":79470,"source_line":149,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::impl$8::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":1462,"record_offset":3424,"parent_record_offset":2564,"ancestor_record_offset":2116,"inlinee":79474,"source_line":565,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_diagonal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1462,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":2116,"inlinee":79471,"source_line":160,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1462,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":2116,"inlinee":79479,"source_line":707,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1462,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":2116,"inlinee":79472,"source_line":78,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$16::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1462,"record_offset":3460,"parent_record_offset":3424,"ancestor_record_offset":2116,"inlinee":79476,"source_line":918,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$22::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1462,"record_offset":3492,"parent_record_offset":3460,"ancestor_record_offset":2116,"inlinee":79478,"source_line":968,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1462,"record_offset":3064,"parent_record_offset":2564,"ancestor_record_offset":2116,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1462,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":2116,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1462,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":2116,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$19::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1462,"record_offset":3544,"parent_record_offset":3516,"ancestor_record_offset":2116,"inlinee":79481,"source_line":1453,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1462,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":2116,"inlinee":27159,"source_line":1577,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":62,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":2020,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":2736,"parent_record_offset":2564,"ancestor_record_offset":2116,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":2116,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":2864,"parent_record_offset":2564,"ancestor_record_offset":2116,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":62,"record_offset":2520,"parent_record_offset":2496,"ancestor_record_offset":2020,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":62,"record_offset":2668,"parent_record_offset":2496,"ancestor_record_offset":2020,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":2644,"parent_record_offset":2564,"ancestor_record_offset":2116,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1462,"record_offset":3780,"parent_record_offset":3752,"ancestor_record_offset":2116,"inlinee":79483,"source_line":180,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":62,"record_offset":2212,"parent_record_offset":2180,"ancestor_record_offset":2020,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::construct_lintels::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/mod.rs
//     evidence={"module":1462,"record_offset":4472,"parent_record_offset":2564,"ancestor_record_offset":2116,"inlinee":79484,"source_line":92,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::curve_to_arch_ws
// 记录形式: inline_instance, inlinee_declaration; instances=27
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x262B
// 首条位置: {"module":62,"inlinee":9771}
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
// 完整结构及原始类型记录: arch_walker.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::arch_walker::ArchWalker
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<true> >", "constructor": "system_wall_constructor::arch_walker::ArchWalker::walk", "arguments": ["system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<true>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >", "constructor": "system_wall_constructor::arch_walker::ArchWalker::walk", "arguments": ["system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> >", "constructor": "system_wall_constructor::arch_walker::ArchWalker::walk", "arguments": ["system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3>", "constructor": "system_wall_constructor::arch_walker::ArchWalker::walk", "arguments": ["system_wall_constructor::construct_arches::construct_arches::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$8>", "constructor": "system_wall_constructor::arch_walker::ArchWalker::walk", "arguments": ["system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$8"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: arch_walker.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/three-version-source-provenance.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/arch_walker.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/arch_walker.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/arch_walker.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: arch_walker.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
// 信息 ID: ca5d5890983483ce3b5c528c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<true> >; module=62, inline_record=2020; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1462, inline_record=2116; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> >; module=1462, inline_record=8380; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3>; module=1462, inline_record=10904; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$8>; module=1462, inline_record=13468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::new
// 信息 ID: 145bafc6a0019a25bde9ac84
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b8e6acdd33b13c966488eca7: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// 引用方: 原程序直接转移 2 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x33fc7e -> 0x7f7d80 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x34498e -> 0x7f7d80 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=394, inline_record=22880; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets; module=420, inline_record=21040; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=3088; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1511, inline_record=3204; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3>
// 信息 ID: 37080e2e5266dc0030fbdfb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::arch_walker::ArchWalker::walk -> ["system_wall_constructor::construct_arches::construct_arches::closure_env$3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ec8b0 -> 0x14105d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >
// 信息 ID: 5e306178b3efaa299a9601fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0 -> ["system_wall_constructor::construct_lintels::construct_lintels::closure_env$2"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::arch_walker::ArchWalker::walk -> ["system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x1479f39 -> 0x140f340 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x147a0d4 -> 0x140f340 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$8>
// 信息 ID: 6801f932b2b02140be3ddd8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::arch_walker::ArchWalker::walk -> ["system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$8"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets @ 0x8332f0 -> 0x1410d30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<true> >
// 信息 ID: ad59b26f266bd8930437e8c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b8e6acdd33b13c966488eca7: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// [类型关系] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6 -> ["true"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::arch_walker::ArchWalker::walk -> ["system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<true>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x33fdac -> 0x1efaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x344abc -> 0x1efaa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> >
// 信息 ID: d0d98766d9d65ea408bda527
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure_env$0 -> ["system_wall_constructor::construct_walls::construct_walls::closure_env$3"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::arch_walker::ArchWalker::walk -> ["system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x14606aa -> 0x140fe70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::curve_to_arch_ws
// 信息 ID: d0a501fe01fdd5f7abcb0c41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 25 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<true> >; module=62, inline_record=540; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<true> >; module=62, inline_record=1088; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<true> >; module=62, inline_record=1416; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<true> >; module=62, inline_record=1992; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<true> >; module=62, inline_record=2796; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1462, inline_record=636; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1462, inline_record=1184; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1462, inline_record=1512; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
